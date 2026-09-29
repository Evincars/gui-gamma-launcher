//! Pre-flight checks for option values, so the UI can flag each bad field
//! instead of surfacing a Python traceback from the launcher.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::spec::{OptKind, PathRule, TextRule};

/// Every reason a request cannot be run.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rejection {
    /// Problems tied to one option, keyed by option key.
    pub(crate) field_errors: BTreeMap<String, String>,
    /// Problems not tied to a single option (unknown command / option).
    pub(crate) errors: Vec<String>,
}

impl Rejection {
    pub(crate) fn is_empty(&self) -> bool {
        self.field_errors.is_empty() && self.errors.is_empty()
    }

    /// Record an error for `key`, keeping the first one if several apply.
    pub(crate) fn field(&mut self, key: &str, message: impl Into<String>) {
        self.field_errors
            .entry(key.to_string())
            .or_insert_with(|| message.into());
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = self.field_errors.iter().map(|(k, m)| format!("`{k}`: {m}"));
        let all: Vec<String> = self.errors.iter().cloned().chain(fields).collect();
        f.write_str(&all.join("\n"))
    }
}

/// `~` / `~/x` → absolute path, mirroring upstream's `Path.expanduser()`.
pub(crate) fn expand_home(raw: &str) -> PathBuf {
    let home = || std::env::var_os("HOME").map(PathBuf::from);
    match raw.strip_prefix('~') {
        Some("") => home().unwrap_or_else(|| raw.into()),
        Some(rest) if rest.starts_with('/') => match home() {
            Some(h) => h.join(&rest[1..]),
            None => raw.into(),
        },
        _ => raw.into(),
    }
}

/// Validate a non-empty, trimmed value for an option of `kind`.
pub(crate) fn check_value(kind: OptKind, value: &str) -> Result<(), String> {
    match kind {
        OptKind::Path(rule) => check_path(&expand_home(value), rule),
        OptKind::Text(rule) => check_text(value, rule),
        OptKind::Bool => Ok(()),
    }
}

fn check_path(path: &Path, rule: PathRule) -> Result<(), String> {
    if !path.is_absolute() {
        return Err("Use an absolute path, e.g. /home/you/Games/Anomaly".into());
    }
    if path.exists() && !path.is_dir() {
        return Err("Points to a file — select a directory".into());
    }
    match rule {
        PathRule::Create => Ok(()),
        PathRule::Existing(_) if !path.is_dir() => Err("Directory does not exist".into()),
        PathRule::Existing(expects) => match expects.iter().find(|e| !path.join(e).exists()) {
            Some(missing) => Err(format!("Not a valid installation: `{missing}` is missing")),
            None => Ok(()),
        },
    }
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')
}

fn check_text(value: &str, rule: TextRule) -> Result<(), String> {
    if value.chars().any(char::is_whitespace) {
        return Err("Must not contain spaces".into());
    }
    let (ok, expected) = match rule {
        TextRule::ModOrganizerTag => (
            value
                .strip_prefix('v')
                .is_some_and(|v| v.starts_with(|c: char| c.is_ascii_digit()) && v.chars().all(is_name_char)),
            "Expected a ModOrganizer release tag, e.g. v2.5.2",
        ),
        TextRule::GitRevision => (
            !value.starts_with(['-', '/'])
                && !value.contains("..")
                && value.chars().all(|c| is_name_char(c) || c == '/'),
            "Expected a commit hash, tag or branch name",
        ),
        TextRule::GithubRepo => {
            if value.contains("://") || value.starts_with("github.com") {
                return Err("Use the owner/repository form, not a URL".into());
            }
            let parts: Vec<&str> = value.split('/').collect();
            (
                parts.len() == 2 && parts.iter().all(|p| !p.is_empty() && p.chars().all(is_name_char)),
                "Expected owner/repository, e.g. Grokitach/Stalker_GAMMA",
            )
        }
    };
    if ok {
        Ok(())
    } else {
        Err(expected.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_rules() {
        assert!(check_text("v2.5.2", TextRule::ModOrganizerTag).is_ok());
        assert!(check_text("2.5.2", TextRule::ModOrganizerTag).is_err());
        assert!(check_text("v 2", TextRule::ModOrganizerTag).is_err());

        assert!(check_text("abc123", TextRule::GitRevision).is_ok());
        assert!(check_text("feature/x", TextRule::GitRevision).is_ok());
        assert!(check_text("--help", TextRule::GitRevision).is_err());
        assert!(check_text("../x", TextRule::GitRevision).is_err());

        assert!(check_text("Grokitach/Stalker_GAMMA", TextRule::GithubRepo).is_ok());
        assert!(check_text("https://github.com/a/b", TextRule::GithubRepo).is_err());
        assert!(check_text("a/b/c", TextRule::GithubRepo).is_err());
        assert!(check_text("a", TextRule::GithubRepo).is_err());
    }

    #[test]
    fn path_rules() {
        let tmp = std::env::temp_dir();
        assert!(check_path(Path::new("relative/dir"), PathRule::Create).is_err());
        assert!(check_path(&tmp.join("gamma-gui-does-not-exist"), PathRule::Create).is_ok());
        assert!(check_path(&tmp.join("gamma-gui-does-not-exist"), PathRule::Existing(&[])).is_err());
        assert!(check_path(&tmp, PathRule::Existing(&[])).is_ok());
        assert!(check_path(&tmp, PathRule::Existing(&["gamma-gui-missing-marker"])).is_err());
    }

    #[test]
    fn expands_home() {
        let home = PathBuf::from(std::env::var_os("HOME").unwrap());
        assert_eq!(expand_home("~"), home);
        assert_eq!(expand_home("~/Games"), home.join("Games"));
        assert_eq!(expand_home("/abs"), PathBuf::from("/abs"));
        assert_eq!(expand_home("~other"), PathBuf::from("~other"));
    }
}
