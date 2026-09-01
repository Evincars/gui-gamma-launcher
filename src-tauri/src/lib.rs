// GUI wrapper for https://github.com/Mord3rca/gamma-launcher
//
// This module is a complete mapper for the `gamma-launcher` v3.1 CLI. Every
// subcommand and every option is described once in `COMMANDS` (the single
// source of truth). The frontend:
//   1. calls `gamma_launcher_schema` to render forms for every command,
//   2. calls `gamma_launcher_preview` to show the exact command line, and
//   3. calls `gamma_launcher_run` to execute the bundled sidecar and stream
//      stdout/stderr back over a channel.
//
// Reference (from `gamma-launcher-v3.1 <cmd> --help`):
//   anomaly-install    --anomaly (req) [--cache-directory] [--anomaly-skip-verify] [--anomaly-purge-cache]
//   check-anomaly      --anomaly (req)
//   check-md5          --gamma (req) [--update-cache] [--remove-unused]
//   full-install       --anomaly (req) --gamma (req) [--cache-directory] [--anomaly-skip-verify]
//                      [--anomaly-purge-cache] [--gamma-no-mod-organizer]
//                      [--gamma-set-mod-organizer-version V] [--custom-gamma-definition V]
//                      [--custom-gamma-repository V] [--no-def-update] [--no-anomaly-patch]
//                      [--preserve-user-config]
//   gamma-setup        --gamma (req) [--cache-directory] [--gamma-no-mod-organizer]
//                      [--gamma-set-mod-organizer-version V]
//   remove-reshade     --anomaly (req)
//   purge-shader-cache --anomaly (req)
//   switch-keymap      --anomaly (req) [--to-dvorak]
//   test-mod-maker     --gamma (req)
//   usvfs-workaround   --anomaly (req) --gamma (req) --final (req)

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

/// Sidecar name as configured in `tauri.conf.json > bundle.externalBin`
/// and allowed in `capabilities/default.json`.
const SIDECAR: &str = "gamma-launcher-v3.1";

// ---------------------------------------------------------------------------
// CLI specification
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum OptKind {
    /// Filesystem path, rendered as a folder picker in the UI.
    Path,
    /// Free text value (e.g. a git tag or repo URL).
    Text,
    /// On/off switch: the flag is passed with no value when enabled.
    Bool,
}

impl OptKind {
    fn as_str(self) -> &'static str {
        match self {
            OptKind::Path => "path",
            OptKind::Text => "text",
            OptKind::Bool => "boolean",
        }
    }
}

#[derive(Clone, Copy)]
struct OptSpec {
    /// Key the frontend uses in the `options` map (camelCase).
    key: &'static str,
    /// The actual CLI flag.
    flag: &'static str,
    kind: OptKind,
    required: bool,
    help: &'static str,
}

struct CmdSpec {
    name: &'static str,
    help: &'static str,
    options: &'static [OptSpec],
}

// Shared option definitions -------------------------------------------------

const ANOMALY: OptSpec = OptSpec {
    key: "anomaly",
    flag: "--anomaly",
    kind: OptKind::Path,
    required: true,
    help: "Path to ANOMALY directory",
};
const GAMMA: OptSpec = OptSpec {
    key: "gamma",
    flag: "--gamma",
    kind: OptKind::Path,
    required: true,
    help: "Path to GAMMA directory",
};
const CACHE_DIRECTORY: OptSpec = OptSpec {
    key: "cacheDirectory",
    flag: "--cache-directory",
    kind: OptKind::Path,
    required: false,
    help: "Path to cache directory",
};
const ANOMALY_SKIP_VERIFY: OptSpec = OptSpec {
    key: "anomalySkipVerify",
    flag: "--anomaly-skip-verify",
    kind: OptKind::Bool,
    required: false,
    help: "Skip installation verification",
};
const ANOMALY_PURGE_CACHE: OptSpec = OptSpec {
    key: "anomalyPurgeCache",
    flag: "--anomaly-purge-cache",
    kind: OptKind::Bool,
    required: false,
    help: "Do not keep 7z archives",
};
const GAMMA_NO_MOD_ORGANIZER: OptSpec = OptSpec {
    key: "gammaNoModOrganizer",
    flag: "--gamma-no-mod-organizer",
    kind: OptKind::Bool,
    required: false,
    help: "Skip ModOrganizer installation",
};
const GAMMA_SET_MOD_ORGANIZER_VERSION: OptSpec = OptSpec {
    key: "gammaSetModOrganizerVersion",
    flag: "--gamma-set-mod-organizer-version",
    kind: OptKind::Text,
    required: false,
    help: "Set ModOrganizer Version (have to match github tags)",
};

static COMMANDS: &[CmdSpec] = &[
    CmdSpec {
        name: "anomaly-install",
        help: "Installation of S.T.A.L.K.E.R.: Anomaly",
        options: &[
            ANOMALY,
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
        ],
    },
    CmdSpec {
        name: "check-anomaly",
        help: "Check Anomaly installation",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "check-md5",
        help: "Check MD5 hash for all addons",
        options: &[
            GAMMA,
            OptSpec {
                key: "updateCache",
                flag: "--update-cache",
                kind: OptKind::Bool,
                required: false,
                help: "Update download cache if file is missing or MD5 do not match",
            },
            OptSpec {
                key: "removeUnused",
                flag: "--remove-unused",
                kind: OptKind::Bool,
                required: false,
                help: "After hash checks, remove unused archive in download directory",
            },
        ],
    },
    CmdSpec {
        name: "full-install",
        help: "Complete install of S.T.A.L.K.E.R.: G.A.M.M.A.",
        options: &[
            ANOMALY,
            GAMMA,
            CACHE_DIRECTORY,
            ANOMALY_SKIP_VERIFY,
            ANOMALY_PURGE_CACHE,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
            OptSpec {
                key: "customGammaDefinition",
                flag: "--custom-gamma-definition",
                kind: OptKind::Text,
                required: false,
                help: "Set a custom revision for S.T.A.L.K.E.R.: G.A.M.M.A.",
            },
            OptSpec {
                key: "customGammaRepository",
                flag: "--custom-gamma-repository",
                kind: OptKind::Text,
                required: false,
                help: "Set a custom repository for S.T.A.L.K.E.R.: G.A.M.M.A.",
            },
            OptSpec {
                key: "noDefUpdate",
                flag: "--no-def-update",
                kind: OptKind::Bool,
                required: false,
                help: "Do not update S.T.A.L.K.E.R.: G.A.M.M.A. definition",
            },
            OptSpec {
                key: "noAnomalyPatch",
                flag: "--no-anomaly-patch",
                kind: OptKind::Bool,
                required: false,
                help: "Do not patch Anomaly directory",
            },
            OptSpec {
                key: "preserveUserConfig",
                flag: "--preserve-user-config",
                kind: OptKind::Bool,
                required: false,
                help: "Do not overwrite user configuration when patching Anomaly directory",
            },
        ],
    },
    CmdSpec {
        name: "gamma-setup",
        help: "Preliminary setup for S.T.A.L.K.E.R.: G.A.M.M.A.",
        options: &[
            GAMMA,
            CACHE_DIRECTORY,
            GAMMA_NO_MOD_ORGANIZER,
            GAMMA_SET_MOD_ORGANIZER_VERSION,
        ],
    },
    CmdSpec {
        name: "remove-reshade",
        help: "Remove ReShade from Anomaly bin",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "purge-shader-cache",
        help: "Purge Anomaly shader cache",
        options: &[ANOMALY],
    },
    CmdSpec {
        name: "switch-keymap",
        help: "Switch keymap of user.ltx from QWERTY to AZERTY layout",
        options: &[
            ANOMALY,
            OptSpec {
                key: "toDvorak",
                flag: "--to-dvorak",
                kind: OptKind::Bool,
                required: false,
                help: "Use DVORAK instead of AZERTY",
            },
        ],
    },
    CmdSpec {
        name: "test-mod-maker",
        help: "Testing mod maker directives",
        options: &[GAMMA],
    },
    CmdSpec {
        name: "usvfs-workaround",
        help: "Workaround to use wine without ModOrganizer (& UserSpace Virtual FileSystem)",
        options: &[
            ANOMALY,
            GAMMA,
            OptSpec {
                key: "final",
                flag: "--final",
                kind: OptKind::Path,
                required: true,
                help: "Path to final install directory",
            },
        ],
    },
];

// ---------------------------------------------------------------------------
// Mapper: (command, options) -> argv
// ---------------------------------------------------------------------------

fn find_command(name: &str) -> Result<&'static CmdSpec, String> {
    COMMANDS
        .iter()
        .find(|c| c.name == name)
        .ok_or_else(|| format!("Unknown gamma-launcher command: `{name}`"))
}

fn value_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Build the full argument vector (`["full-install", "--anomaly", "/path", ...]`)
/// for the given command and option map. Validates required options and rejects
/// unknown keys so UI typos surface early instead of as a cryptic argparse error.
fn build_args(command: &str, options: &Map<String, Value>) -> Result<Vec<String>, String> {
    let spec = find_command(command)?;

    for key in options.keys() {
        if !spec.options.iter().any(|o| o.key == key) {
            return Err(format!(
                "Unknown option `{key}` for command `{command}`"
            ));
        }
    }

    let mut args = vec![command.to_string()];
    for opt in spec.options {
        match opt.kind {
            OptKind::Bool => {
                let enabled = options
                    .get(opt.key)
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
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

fn quote_for_display(arg: &str) -> String {
    if arg.is_empty() || arg.chars().any(char::is_whitespace) {
        format!("\"{}\"", arg.replace('"', "\\\""))
    } else {
        arg.to_string()
    }
}

// ---------------------------------------------------------------------------
// IPC payloads
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RunRequest {
    command: String,
    #[serde(default)]
    options: Map<String, Value>,
}

#[derive(Clone, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum RunEvent {
    Started { command: String, args: Vec<String> },
    Stdout { line: String },
    Stderr { line: String },
    Error { message: String },
    Finished { code: Option<i32>, signal: Option<i32>, success: bool },
}

#[derive(Serialize)]
pub struct RunResult {
    code: Option<i32>,
    signal: Option<i32>,
    success: bool,
    stdout: String,
    stderr: String,
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Full machine-readable description of every command and option, so the UI can
/// generate all forms without hardcoding anything.
#[tauri::command]
fn gamma_launcher_schema() -> Value {
    let commands: Vec<Value> = COMMANDS
        .iter()
        .map(|c| {
            let options: Vec<Value> = c
                .options
                .iter()
                .map(|o| {
                    json!({
                        "key": o.key,
                        "flag": o.flag,
                        "type": o.kind.as_str(),
                        "required": o.required,
                        "description": o.help,
                    })
                })
                .collect();
            json!({
                "name": c.name,
                "description": c.help,
                "options": options,
            })
        })
        .collect();

    json!({
        "binary": SIDECAR,
        "commands": commands,
    })
}

/// Return the exact command line that `gamma_launcher_run` would execute, for
/// display / confirmation in the UI. Also performs full validation.
#[tauri::command]
fn gamma_launcher_preview(request: RunRequest) -> Result<String, String> {
    let args = build_args(&request.command, &request.options)?;
    let rendered = std::iter::once(SIDECAR.to_string())
        .chain(args)
        .map(|a| quote_for_display(&a))
        .collect::<Vec<_>>()
        .join(" ");
    Ok(rendered)
}

/// Raw argv form of a request (`["full-install", "--anomaly", "/path", ...]`).
#[tauri::command]
fn gamma_launcher_args(request: RunRequest) -> Result<Vec<String>, String> {
    build_args(&request.command, &request.options)
}

/// `gamma-launcher --version`.
#[tauri::command]
async fn gamma_launcher_version(app: AppHandle) -> Result<String, String> {
    let output = app
        .shell()
        .sidecar(SIDECAR)
        .map_err(|e| e.to_string())?
        .arg("--version")
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

/// Execute a command. stdout/stderr are streamed line-by-line through `on_event`
/// as they arrive; the aggregated output and exit status are also returned when
/// the process terminates.
#[tauri::command]
async fn gamma_launcher_run(
    app: AppHandle,
    request: RunRequest,
    on_event: Channel<RunEvent>,
) -> Result<RunResult, String> {
    let args = build_args(&request.command, &request.options)?;

    let (mut rx, _child) = app
        .shell()
        .sidecar(SIDECAR)
        .map_err(|e| e.to_string())?
        .args(&args)
        .spawn()
        .map_err(|e| e.to_string())?;

    let _ = on_event.send(RunEvent::Started {
        command: request.command.clone(),
        args: args.clone(),
    });

    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut code = None;
    let mut signal = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => {
                let line = String::from_utf8_lossy(&bytes).into_owned();
                stdout.push_str(&line);
                stdout.push('\n');
                let _ = on_event.send(RunEvent::Stdout { line });
            }
            CommandEvent::Stderr(bytes) => {
                let line = String::from_utf8_lossy(&bytes).into_owned();
                stderr.push_str(&line);
                stderr.push('\n');
                let _ = on_event.send(RunEvent::Stderr { line });
            }
            CommandEvent::Error(message) => {
                stderr.push_str(&message);
                stderr.push('\n');
                let _ = on_event.send(RunEvent::Error { message });
            }
            CommandEvent::Terminated(payload) => {
                code = payload.code;
                signal = payload.signal;
            }
            _ => {}
        }
    }

    let success = code == Some(0);
    let _ = on_event.send(RunEvent::Finished {
        code,
        signal,
        success,
    });

    Ok(RunResult {
        code,
        signal,
        success,
        stdout,
        stderr,
    })
}

// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            gamma_launcher_schema,
            gamma_launcher_preview,
            gamma_launcher_args,
            gamma_launcher_version,
            gamma_launcher_run,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
        for spec in COMMANDS {
            // Build with only required options filled with a dummy value.
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
        let err = build_args(
            "check-anomaly",
            &opts(json!({ "anomaly": "/a", "bogus": true })),
        )
        .unwrap_err();
        assert!(err.contains("bogus"));
    }

    #[test]
    fn unknown_command_errors() {
        assert!(build_args("nope", &Map::new()).is_err());
    }
}
