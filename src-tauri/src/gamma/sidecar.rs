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

/// Actionable explanation for a known launcher failure, if `output` shows one.
pub(crate) fn diagnose(output: &str) -> Option<&'static str> {
    output
        .contains("Couldn't find path to unrar library")
        .then_some(UNRAR_HINT)
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
}
