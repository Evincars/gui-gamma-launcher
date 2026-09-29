//! Maps a `(command, options)` request coming from the UI into the exact
//! argument vector to hand to the `gamma-launcher` sidecar.

use std::path::PathBuf;

use serde_json::{Map, Value};

use super::spec::{find_command, CmdSpec, OptKind, OptSpec};
use super::validate::{check_value, expand_home, Rejection};

/// Build the full argument vector (`["full-install", "--anomaly", "/path", ...]`),
/// collecting every validation problem instead of stopping at the first.
pub(crate) fn build_args(command: &str, options: &Map<String, Value>) -> Result<Vec<String>, Rejection> {
    let mut rejection = Rejection::default();
    let Some(spec) = find_command(command) else {
        rejection.errors.push(format!("Unknown gamma-launcher command `{command}`"));
        return Err(rejection);
    };

    for key in options.keys().filter(|k| !spec.options.iter().any(|o| o.key == *k)) {
        rejection.errors.push(format!("Unknown option `{key}` for `{command}`"));
    }

    let mut args = vec![command.to_string()];
    for opt in spec.options {
        match map_option(opt, options.get(opt.key)) {
            Ok(fragment) => args.extend(fragment),
            Err(message) => rejection.field(opt.key, message),
        }
    }
    check_distinct_dirs(spec, options, &mut rejection);

    if rejection.is_empty() {
        Ok(args)
    } else {
        Err(rejection)
    }
}

/// Validate one option and return the argv fragment it contributes.
fn map_option(opt: &OptSpec, value: Option<&Value>) -> Result<Vec<String>, String> {
    if let OptKind::Bool = opt.kind {
        return match value {
            None | Some(Value::Null | Value::Bool(false)) => Ok(vec![]),
            Some(Value::Bool(true)) => Ok(vec![opt.flag.to_string()]),
            Some(_) => Err("Expected true or false".into()),
        };
    }

    let text = match value {
        None | Some(Value::Null) => "",
        Some(Value::String(s)) => s.trim(),
        Some(_) => return Err("Expected text".into()),
    };
    if text.is_empty() {
        return if opt.required {
            Err("This field is required".into())
        } else {
            Ok(vec![])
        };
    }
    check_value(opt.kind, text)?;
    Ok(vec![opt.flag.to_string(), text.to_string()])
}

/// Anomaly, GAMMA and final install dirs must not overlap each other.
fn check_distinct_dirs(spec: &CmdSpec, options: &Map<String, Value>, rejection: &mut Rejection) {
    let mut seen: Vec<(PathBuf, &OptSpec)> = Vec::new();
    for opt in spec
        .options
        .iter()
        .filter(|o| o.required && matches!(o.kind, OptKind::Path(_)))
    {
        let Some(raw) = options
            .get(opt.key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let path = expand_home(raw);
        match seen.iter().find(|(p, _)| *p == path) {
            Some((_, other)) => rejection.field(opt.key, format!("Must differ from {}", other.flag)),
            None => seen.push((path, opt)),
        }
    }
}

/// Quote a single argument so the displayed command line can be pasted into a shell.
pub(crate) fn shell_quote(arg: &str) -> String {
    let safe = |c: char| c.is_ascii_alphanumeric() || "-_./=:@%+,".contains(c);
    if !arg.is_empty() && arg.chars().all(safe) {
        arg.to_string()
    } else {
        format!("'{}'", arg.replace('\'', r"'\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamma::spec::{PathRule, COMMANDS};
    use serde_json::json;

    fn opts(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    /// A directory satisfying every `PathRule::Existing` marker in the spec.
    fn fixture_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("gamma-gui-test-{name}-{}", std::process::id()));
        for spec in COMMANDS {
            for o in spec.options {
                if let OptKind::Path(PathRule::Existing(markers)) = o.kind {
                    for m in markers {
                        std::fs::create_dir_all(dir.join(m)).unwrap();
                    }
                }
            }
        }
        dir
    }

    #[test]
    fn every_command_is_reachable() {
        for spec in COMMANDS {
            let mut m = Map::new();
            for o in spec.options.iter().filter(|o| o.required) {
                m.insert(o.key.into(), json!(fixture_dir(o.key).to_str().unwrap()));
            }
            let args = build_args(spec.name, &m).unwrap_or_else(|e| panic!("{}: {e}", spec.name));
            assert_eq!(args[0], spec.name);
        }
    }

    #[test]
    fn full_install_maps_all_options() {
        let args = build_args(
            "full-install",
            &opts(json!({
                "anomaly": "/games/anomaly",
                "gamma": "/games/gamma",
                "cacheDirectory": "/cache",
                "anomalySkipVerify": true,
                "anomalyPurgeCache": false,
                "gammaNoModOrganizer": true,
                "gammaSetModOrganizerVersion": "v2.5.2",
                "customGammaDefinition": "abc123",
                "customGammaRepository": "Grokitach/Stalker_GAMMA",
                "noDefUpdate": true,
                "noAnomalyPatch": true,
                "preserveUserConfig": true,
            })),
        )
        .unwrap();

        assert_eq!(
            args,
            vec![
                "full-install",
                "--anomaly",
                "/games/anomaly",
                "--gamma",
                "/games/gamma",
                "--cache-directory",
                "/cache",
                "--anomaly-skip-verify",
                "--gamma-no-mod-organizer",
                "--gamma-set-mod-organizer-version",
                "v2.5.2",
                "--custom-gamma-definition",
                "abc123",
                "--custom-gamma-repository",
                "Grokitach/Stalker_GAMMA",
                "--no-def-update",
                "--no-anomaly-patch",
                "--preserve-user-config",
            ]
        );
    }

    #[test]
    fn collects_all_field_errors() {
        let err = build_args(
            "full-install",
            &opts(json!({
                "anomaly": "relative",
                "gammaSetModOrganizerVersion": "2.5.2",
                "customGammaRepository": "https://github.com/a/b",
            })),
        )
        .unwrap_err();
        let keys: Vec<&str> = err.field_errors.keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            ["anomaly", "customGammaRepository", "gamma", "gammaSetModOrganizerVersion"]
        );
    }

    #[test]
    fn same_dir_for_anomaly_and_gamma_errors() {
        let err = build_args(
            "full-install",
            &opts(json!({ "anomaly": "/games/x", "gamma": "/games/x/" })),
        )
        .unwrap_err();
        assert!(err.field_errors["gamma"].contains("--anomaly"));
    }

    #[test]
    fn unknown_option_and_command_error() {
        let err = build_args("check-anomaly", &opts(json!({ "bogus": true }))).unwrap_err();
        assert!(err.errors[0].contains("bogus"));
        assert!(build_args("nope", &Map::new()).is_err());
    }

    #[test]
    fn wrong_value_types_error() {
        let err = build_args(
            "full-install",
            &opts(json!({ "anomaly": 1, "gamma": "/g", "noDefUpdate": "yes" })),
        )
        .unwrap_err();
        assert!(err.field_errors.contains_key("anomaly"));
        assert!(err.field_errors.contains_key("noDefUpdate"));
    }

    #[test]
    fn quotes_for_shell() {
        assert_eq!(shell_quote("/games/S.T.A.L.K.E.R"), "/games/S.T.A.L.K.E.R");
        assert_eq!(shell_quote("/my games/gamma"), "'/my games/gamma'");
        assert_eq!(shell_quote("it's"), r"'it'\''s'");
        assert_eq!(shell_quote(""), "''");
    }
}
