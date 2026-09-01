//! Maps a `(command, options)` request coming from the UI into the exact
//! argument vector to hand to the `gamma-launcher` sidecar.

use serde_json::{Map, Value};

use super::spec::{find_command, OptKind};

fn value_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Build the full argument vector
/// (`["full-install", "--anomaly", "/path", ...]`) for the given command.
///
/// Required options are validated and unknown keys are rejected, so UI typos
/// surface here instead of as a cryptic argparse failure.
pub(crate) fn build_args(command: &str, options: &Map<String, Value>) -> Result<Vec<String>, String> {
    let spec = find_command(command)?;

    for key in options.keys() {
        if !spec.options.iter().any(|o| o.key == key) {
            return Err(format!("Unknown option `{key}` for command `{command}`"));
        }
    }

    let mut args = vec![command.to_string()];
    for opt in spec.options {
        match opt.kind {
            OptKind::Bool => {
                let enabled = options.get(opt.key).and_then(Value::as_bool).unwrap_or(false);
                if enabled {
                    args.push(opt.flag.to_string());
                }
            }
            OptKind::Path | OptKind::Text => {
                let raw = options.get(opt.key).and_then(value_to_string);
                let value = raw.as_deref().map(str::trim).unwrap_or("");
                if value.is_empty() {
                    if opt.required {
                        return Err(format!(
                            "Missing required option `{}` for command `{command}`",
                            opt.key
                        ));
                    }
                } else {
                    args.push(opt.flag.to_string());
                    args.push(value.to_string());
                }
            }
        }
    }
    Ok(args)
}

/// Quote a single argument for readable display of the full command line.
pub(crate) fn quote_for_display(arg: &str) -> String {
    if arg.is_empty() || arg.chars().any(char::is_whitespace) {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn opts(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn every_command_is_reachable() {
        for spec in super::super::spec::COMMANDS {
            let mut m = Map::new();
            for o in spec.options {
                if o.required {
                    m.insert(o.key.to_string(), json!("/tmp/x"));
                }
            }
            let args = build_args(spec.name, &m).expect(spec.name);
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
                "customGammaRepository": "https://example/repo.git",
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
                "https://example/repo.git",
                "--no-def-update",
                "--no-anomaly-patch",
                "--preserve-user-config",
            ]
        );
    }

    #[test]
    fn missing_required_option_errors() {
        let err = build_args("check-anomaly", &Map::new()).unwrap_err();
        assert!(err.contains("anomaly"));
    }

    #[test]
    fn unknown_option_errors() {
        let err = build_args("check-anomaly", &opts(json!({ "anomaly": "/a", "bogus": true })))
            .unwrap_err();
        assert!(err.contains("bogus"));
    }

    #[test]
    fn unknown_command_errors() {
        assert!(build_args("nope", &Map::new()).is_err());
    }

    #[test]
    fn quotes_paths_with_spaces() {
        assert_eq!(quote_for_display("/games/S.T.A.L.K.E.R"), "/games/S.T.A.L.K.E.R");
        assert_eq!(quote_for_display("/my games/gamma"), "\"/my games/gamma\"");
    }
}
