//! Host checks for everything the bundled launcher needs at runtime but does not ship:
//! libunrar (RAR), `7z` (BCJ2 archives), `git` (repositories), CA certificates (HTTPS)
//! and enough space in TMPDIR.

use std::path::{Path, PathBuf};

use serde::Serialize;
use tauri::AppHandle;

use super::sidecar;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Warning,
    Error,
}

#[derive(Serialize)]
pub struct Requirement {
    id: &'static str,
    label: &'static str,
    status: Status,
    detail: String,
    /// How to fix it; only set when `status` is not `Ok`.
    hint: Option<&'static str>,
}

#[derive(Serialize)]
pub struct Requirements {
    /// `gamma-launcher --version` output, if the launcher starts.
    version: Option<String>,
    checks: Vec<Requirement>,
}

const GIB: u64 = 1024 * 1024 * 1024;
/// Upstream `check_tmp_free_space`: gamma-setup (also run by a first full-install) / full-install.
const TMP_NEEDED_GIB: (u64, u64) = (12, 6);

const SEVEN_ZIP_HINT: &str = "Used for 7z archives with BCJ2 compression (several G.A.M.M.A. mods).
  • Fedora: sudo dnf install 7zip
  • Ubuntu / Debian: sudo apt install 7zip
  • Arch / Manjaro: sudo pacman -S 7zip";

const GIT_HINT: &str = "Without git the launcher falls back to downloading full archives (slower, no incremental updates).
  • Fedora: sudo dnf install git
  • Ubuntu / Debian: sudo apt install git
  • Arch / Manjaro: sudo pacman -S git";

const CERTS_HINT: &str = "HTTPS downloads will fail without a CA bundle.
Install your distribution's ca-certificates package, or set SSL_CERT_FILE / SSL_CERT_DIR.";

const TMP_HINT: &str = "Mods are extracted in TMPDIR. Start this app with TMPDIR pointing to a folder on a large disk,
e.g. `mkdir -p ~/.cache/gamma-tmp && TMPDIR=~/.cache/gamma-tmp gui-gamma-launcher`.
On Fedora 44+ /tmp is a tmpfs that may also enforce a per-user quota (Errno 122).";

const LAUNCHER_HINT: &str = "The bundled gamma-launcher binary could not start; see the message above.";

/// CA locations probed by upstream `launcher/bootstrap.py`.
const CA_FILES: &[&str] = &[
    "/etc/ssl/certs/ca-certificates.crt",
    "/etc/pki/tls/certs/ca-bundle.crt",
    "/etc/ssl/ca-bundle.pem",
    "/etc/pki/tls/cacert.pem",
    "/etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem",
    "/etc/ssl/cert.pem",
];
const CA_DIRS: &[&str] = &["/etc/ssl/certs", "/etc/tls/pki/certs"];

fn requirement(id: &'static str, label: &'static str, status: Status, detail: String, hint: &'static str) -> Requirement {
    Requirement {
        id,
        label,
        status,
        detail,
        hint: (status != Status::Ok).then_some(hint),
    }
}

/// Run every check. The launcher is started once (`--version`), which also proves libunrar loads.
pub(crate) async fn check(app: &AppHandle) -> Requirements {
    let launcher = sidecar::version(app).await;
    let unrar_missing = matches!(&launcher, Err(e) if e.as_str() == sidecar::UNRAR_HINT);

    let (launcher_check, unrar_check) = match &launcher {
        Ok(v) => (
            requirement("launcher", "gamma-launcher", Status::Ok, v.clone(), ""),
            requirement("libunrar", "libunrar (RAR extraction)", Status::Ok, "Loaded by the launcher".into(), ""),
        ),
        Err(_) if unrar_missing => (
            requirement("launcher", "gamma-launcher", Status::Error, "Cannot start: libunrar is missing".into(), LAUNCHER_HINT),
            requirement("libunrar", "libunrar (RAR extraction)", Status::Error, "Not found".into(), sidecar::UNRAR_HINT),
        ),
        Err(e) => (
            requirement("launcher", "gamma-launcher", Status::Error, e.clone(), LAUNCHER_HINT),
            requirement("libunrar", "libunrar (RAR extraction)", Status::Warning, "Unknown — the launcher did not start".into(), sidecar::UNRAR_HINT),
        ),
    };

    Requirements {
        version: launcher.ok(),
        checks: vec![
            launcher_check,
            unrar_check,
            executable_check("7z", "7-Zip (7z)", Status::Error, SEVEN_ZIP_HINT),
            executable_check("git", "git", Status::Warning, GIT_HINT),
            ca_check(),
            tmp_check(),
        ],
    }
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|p| is_executable(p))
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}

fn executable_check(name: &'static str, label: &'static str, missing: Status, hint: &'static str) -> Requirement {
    match find_in_path(name) {
        Some(p) => requirement(name, label, Status::Ok, p.display().to_string(), hint),
        None => requirement(name, label, missing, format!("`{name}` not found in PATH"), hint),
    }
}

fn ca_check() -> Requirement {
    let from_env = ["SSL_CERT_FILE", "SSL_CERT_DIR"]
        .iter()
        .find_map(|k| std::env::var(k).ok().map(|v| format!("{k}={v}")));
    let found = from_env.or_else(|| {
        CA_FILES
            .iter()
            .find(|f| Path::new(f).is_file())
            .or_else(|| CA_DIRS.iter().find(|d| Path::new(d).is_dir()))
            .map(|p| p.to_string())
    });
    match found {
        Some(p) => requirement("certs", "CA certificates (HTTPS)", Status::Ok, p, CERTS_HINT),
        None => requirement("certs", "CA certificates (HTTPS)", Status::Error, "No CA bundle found".into(), CERTS_HINT),
    }
}

/// Free bytes for unprivileged users and whether the filesystem is a tmpfs.
#[cfg(target_os = "linux")]
#[allow(clippy::unnecessary_cast)] // field types differ between architectures
fn fs_usage(path: &Path) -> Option<(u64, bool)> {
    use std::os::unix::ffi::OsStrExt;
    let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    // SAFETY: `statfs` only writes into the zeroed struct we pass; the path is NUL-terminated.
    let mut s: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path.as_ptr(), &mut s) } != 0 {
        return None;
    }
    let free = (s.f_bavail as u64).saturating_mul(s.f_bsize as u64);
    Some((free, s.f_type as i64 == libc::TMPFS_MAGIC as i64))
}

#[cfg(not(target_os = "linux"))]
fn fs_usage(_path: &Path) -> Option<(u64, bool)> {
    None
}

fn tmp_status(free_gib: u64, tmpfs: bool) -> Status {
    let (setup, install) = TMP_NEEDED_GIB;
    if free_gib < install {
        Status::Error
    } else if free_gib < setup || tmpfs {
        Status::Warning
    } else {
        Status::Ok
    }
}

fn tmp_check() -> Requirement {
    let dir = std::env::temp_dir();
    let label = "Temporary directory space";
    let Some((free, tmpfs)) = fs_usage(&dir) else {
        return requirement("tmp", label, Status::Warning, format!("Could not read free space of {}", dir.display()), TMP_HINT);
    };
    let free_gib = free / GIB;
    let (setup, install) = TMP_NEEDED_GIB;
    let detail = format!(
        "{} — {free_gib} GiB free{} (needs {install} GiB, {setup} GiB for the first install)",
        dir.display(),
        if tmpfs { ", tmpfs" } else { "" },
    );
    requirement("tmp", label, tmp_status(free_gib, tmpfs), detail, TMP_HINT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_thresholds() {
        assert_eq!(tmp_status(5, false), Status::Error);
        assert_eq!(tmp_status(8, false), Status::Warning);
        assert_eq!(tmp_status(20, false), Status::Ok);
        assert_eq!(tmp_status(20, true), Status::Warning);
    }

    #[test]
    fn finds_common_executables() {
        assert!(find_in_path("sh").is_some());
        assert!(find_in_path("definitely-not-a-real-binary-xyz").is_none());
    }

    #[test]
    fn hint_only_when_not_ok() {
        assert!(requirement("x", "x", Status::Ok, String::new(), "h").hint.is_none());
        assert_eq!(requirement("x", "x", Status::Warning, String::new(), "h").hint, Some("h"));
    }
}
