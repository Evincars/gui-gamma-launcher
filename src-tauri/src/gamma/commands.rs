//! Tauri commands exposed to the frontend.

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use tauri::ipc::Channel;
use tauri::{AppHandle, State};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

use super::mapper::{build_args, quote_for_display};
use super::spec::COMMANDS;
use super::SIDECAR;

/// The single currently-running sidecar process, if any.
///
/// `gamma-launcher` operations (especially `full-install`) are long-running and
/// mutate game directories, so only one runs at a time and it can be cancelled.
#[derive(Default)]
pub struct ActiveRun(Mutex<Option<CommandChild>>);

// -- IPC payloads ------------------------------------------------------------

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

// -- Commands --------------------------------------------------------------

/// Full machine-readable description of every command and option, so the UI can
/// generate all forms without hardcoding anything.
#[tauri::command]
pub fn gamma_launcher_schema() -> Value {
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

    json!({ "binary": SIDECAR, "commands": commands })
}

/// The exact command line `gamma_launcher_run` would execute, for display /
/// confirmation. Also performs full validation.
#[tauri::command]
pub fn gamma_launcher_preview(request: RunRequest) -> Result<String, String> {
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
pub fn gamma_launcher_args(request: RunRequest) -> Result<Vec<String>, String> {
    build_args(&request.command, &request.options)
}

/// `gamma-launcher --version`.
#[tauri::command]
pub async fn gamma_launcher_version(app: AppHandle) -> Result<String, String> {
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
pub async fn gamma_launcher_run(
    app: AppHandle,
    state: State<'_, ActiveRun>,
    request: RunRequest,
    on_event: Channel<RunEvent>,
) -> Result<RunResult, String> {
    let args = build_args(&request.command, &request.options)?;

    if state.0.lock().unwrap().is_some() {
        return Err("A gamma-launcher command is already running".into());
    }

    let (mut rx, child) = app
        .shell()
        .sidecar(SIDECAR)
        .map_err(|e| e.to_string())?
        .args(&args)
        .spawn()
        .map_err(|e| e.to_string())?;

    *state.0.lock().unwrap() = Some(child);

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

    *state.0.lock().unwrap() = None;

    let success = code == Some(0);
    let _ = on_event.send(RunEvent::Finished { code, signal, success });

    Ok(RunResult {
        code,
        signal,
        success,
        stdout,
        stderr,
    })
}

/// Kill the currently-running command, if any. Returns `true` if a process was
/// actually terminated.
#[tauri::command]
pub fn gamma_launcher_cancel(state: State<'_, ActiveRun>) -> Result<bool, String> {
    let child = state.0.lock().unwrap().take();
    match child {
        Some(child) => {
            child.kill().map_err(|e| e.to_string())?;
            Ok(true)
        }
        None => Ok(false),
    }
}
