//! Everything about invoking the bundled `gamma-launcher` binary.

use tauri::AppHandle;
use tauri_plugin_shell::process::Command;
use tauri_plugin_shell::ShellExt;

/// Sidecar name as configured in `tauri.conf.json > bundle.externalBin`
/// (Tauri appends the target triple when bundling).
pub(crate) const NAME: &str = "gamma-launcher-v3.1";

/// A ready-to-configure sidecar command.
pub(crate) fn command(app: &AppHandle) -> Result<Command, String> {
    let cmd = app.shell().sidecar(NAME).map_err(|e| e.to_string())?;
    Ok(cmd
        // The GUI stores its own settings; the launcher's config.ini would inject stale args.
        .env("GAMMA_LAUNCHER_NO_CONFIG", "1")
        // Python block-buffers piped stdout, which would stall live output.
        .env("PYTHONUNBUFFERED", "1"))
}

const UNRAR_HINT: &str = "gamma-launcher needs the unrar library (libunrar), which was not found.
Install it, then try again:
  • Fedora (RPM Fusion non-free): sudo dnf install libunrar
  • Ubuntu: sudo apt install libunrar5t64
  • Debian (non-free): sudo apt install libunrar5
  • Arch / Manjaro: sudo pacman -S libunrar
Alternatively set UNRAR_LIB_PATH to an existing libunrar.so.";

const TMPDIR_HINT: &str = "Not enough space in the temporary directory (full-install needs 6 GiB, gamma-setup 12 GiB).
On Fedora /tmp is a size-limited tmpfs. Start this app with TMPDIR pointing to a folder on a larger disk,
e.g. `mkdir -p ~/.cache/gamma-tmp && TMPDIR=~/.cache/gamma-tmp gui-gamma-launcher`.";

const QUOTA_HINT: &str = "Disk quota exceeded — usually the per-user tmpfs quota on /tmp (Fedora 44+).
Start this app with TMPDIR pointing to a folder on a real disk (see above),
or lift the quota until reboot: sudo setquota -u $USER 0 0 0 0 /tmp";

const DOWNLOAD_HINT: &str = "A download failed (server error or ModDB/Cloudflare block).
Open the URL shown above in a browser and save the file into <GAMMA>/downloads/
(GitHub archives are named <project>-<file>, e.g. anomaly-exo-latest.zip), then run again —
cached files are reused. For repeated ModDB blocks, retry later or via a VPN.";

/// Known failure signatures (from upstream sources & issues) and what to do about them.
const KNOWN_FAILURES: &[(&str, &str)] = &[
    ("Couldn't find path to unrar library", UNRAR_HINT),
    ("of space in TMPDIR", TMPDIR_HINT),
    ("[Errno 122]", QUOTA_HINT),
    ("requests.exceptions.HTTPError", DOWNLOAD_HINT),
    ("ModDBDownloadError", DOWNLOAD_HINT),
];

/// Actionable explanation for a known launcher failure, if `output` shows one.
pub(crate) fn diagnose(output: &str) -> Option<&'static str> {
    KNOWN_FAILURES
        .iter()
        .find(|(needle, _)| output.contains(needle))
        .map(|(_, hint)| *hint)
}

/// Short, human-readable reason for a failed launcher invocation.
pub(crate) fn failure_reason(stderr: &str) -> String {
    diagnose(stderr)
        .map(str::to_string)
        .or_else(|| stderr.lines().rev().find(|l| !l.trim().is_empty()).map(str::to_string))
        .unwrap_or_else(|| "gamma-launcher exited with an error".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explains_missing_unrar() {
        let traceback = "Traceback (most recent call last):\n  File \"unrar/unrarlib.py\", line 57\nLookupError: Couldn't find path to unrar library.\n[PYI-1:ERROR] Failed to execute script";
        assert_eq!(failure_reason(traceback), UNRAR_HINT);
        assert_eq!(failure_reason("boom\nlast line\n\n"), "last line");
        assert!(!failure_reason("").is_empty());
    }

    #[test]
    fn recognises_upstream_failures() {
        let cases = [
            ("RuntimeError: You need at least 12 GiB of space in TMPDIR for this to work.", TMPDIR_HINT),
            ("OSError: [Errno 122] Disk quota exceeded", QUOTA_HINT),
            ("requests.exceptions.HTTPError: 504 Server Error: Gateway Time-out for url: x", DOWNLOAD_HINT),
        ];
        for (line, hint) in cases {
            assert_eq!(diagnose(line), Some(hint), "{line}");
        }
        assert_eq!(diagnose("[+] Installing base Anomaly 1.5.3"), None);
    }
}
