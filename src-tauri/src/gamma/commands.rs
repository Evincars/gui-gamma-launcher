//! Tauri commands exposed to the frontend — a thin layer over the other modules.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tauri::ipc::Channel;
use tauri::{AppHandle, State};

use super::mapper::{build_args, shell_quote};
use super::requirements::{self, Requirements};
use super::runner::{self, ActiveRun, RunEvent, RunResult};
use super::schema::{self, Schema};
use super::sidecar;
use super::validate::Rejection;

#[derive(Debug, Deserialize)]
pub struct RunRequest {
    command: String,
    #[serde(default)]
    options: Map<String, Value>,
}

/// Outcome of validating a request: either a runnable command line or the problems.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    command_line: Option<String>,
    #[serde(flatten)]
    rejection: Rejection,
}

/// Full machine-readable description of every command and option.
#[tauri::command]
pub fn gamma_launcher_schema() -> Schema {
    schema::schema()
}

/// Validate a request and, if valid, render the exact command line that would run.
#[tauri::command]
pub fn gamma_launcher_validate(request: RunRequest) -> Validation {
    match build_args(&request.command, &request.options) {
        Ok(args) => Validation {
            command_line: Some(
                std::iter::once(sidecar::NAME.to_string())
                    .chain(args)
                    .map(|a| shell_quote(&a))
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            rejection: Rejection::default(),
        },
        Err(rejection) => Validation {
            command_line: None,
            rejection,
        },
    }
}

/// Host dependency checks, including whether the launcher itself starts.
#[tauri::command]
pub async fn gamma_launcher_requirements(app: AppHandle) -> Requirements {
    requirements::check(&app).await
}

/// Execute a command, streaming stdout/stderr through `on_event`.
#[tauri::command]
pub async fn gamma_launcher_run(
    app: AppHandle,
    state: State<'_, ActiveRun>,
    request: RunRequest,
    on_event: Channel<RunEvent>,
) -> Result<RunResult, String> {
    let args = build_args(&request.command, &request.options).map_err(|r| r.to_string())?;
    runner::run(&app, &state, args, &on_event).await
}

/// Kill the currently-running command. Returns `true` if one was terminated.
#[tauri::command]
pub fn gamma_launcher_cancel(state: State<'_, ActiveRun>) -> Result<bool, String> {
    state.cancel()
}
