//! Runs the sidecar and streams its output. At most one run at a time.

use std::sync::{Mutex, MutexGuard, PoisonError};

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

use super::sidecar;

/// The single currently-running sidecar process, if any.
///
/// `gamma-launcher` operations (especially `full-install`) are long-running and
/// mutate game directories, so only one runs at a time and it can be cancelled.
#[derive(Default)]
pub struct ActiveRun(Mutex<Option<CommandChild>>);

impl ActiveRun {
    fn slot(&self) -> MutexGuard<'_, Option<CommandChild>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Kill the running process. Returns `true` if there was one.
    pub(crate) fn cancel(&self) -> Result<bool, String> {
        match self.slot().take() {
            Some(child) => child.kill().map(|_| true).map_err(|e| e.to_string()),
            None => Ok(false),
        }
    }
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
}

/// Drop the line terminator and keep only the last `\r`-overwritten segment
/// (progress bars redraw a line with carriage returns).
fn clean_line(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let text = text.trim_end_matches(['\r', '\n']);
    text.rsplit('\r').next().unwrap_or_default().to_string()
}

/// Spawn `args` (already validated) and forward output through `on_event`
/// until the process terminates.
pub(crate) async fn run(
    app: &AppHandle,
    active: &ActiveRun,
    args: Vec<String>,
    on_event: &Channel<RunEvent>,
) -> Result<RunResult, String> {
    let mut rx = {
        // Check-and-store under one lock so two clicks can't start two runs.
        let mut slot = active.slot();
        if slot.is_some() {
            return Err("A gamma-launcher command is already running".into());
        }
        let (rx, child) = sidecar::command(app)?
            .args(&args)
            .spawn()
            .map_err(|e| e.to_string())?;
        *slot = Some(child);
        rx
    };

    let send = |event| {
        let _ = on_event.send(event);
    };
    send(RunEvent::Started {
        command: args[0].clone(),
        args: args.clone(),
    });

    let mut hint = None;
    let (mut code, mut signal) = (None, None);
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => send(RunEvent::Stdout { line: clean_line(&bytes) }),
            CommandEvent::Stderr(bytes) => {
                let line = clean_line(&bytes);
                hint = hint.or_else(|| sidecar::diagnose(&line));
                send(RunEvent::Stderr { line });
            }
            CommandEvent::Error(message) => send(RunEvent::Error { message }),
            CommandEvent::Terminated(payload) => {
                code = payload.code;
                signal = payload.signal;
            }
            _ => {}
        }
    }
    active.slot().take();

    let success = code == Some(0);
    if let (false, Some(message)) = (success, hint) {
        send(RunEvent::Error { message: message.into() });
    }
    send(RunEvent::Finished { code, signal, success });

    Ok(RunResult { code, signal, success })
}

#[cfg(test)]
mod tests {
    use super::clean_line;

    #[test]
    fn cleans_lines() {
        assert_eq!(clean_line(b"hello\n"), "hello");
        assert_eq!(clean_line(b"hello\r\n"), "hello");
        assert_eq!(clean_line(b" 10%\r 50%\r100%\n"), "100%");
    }
}
