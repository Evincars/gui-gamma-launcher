//! GUI wrapper for https://github.com/Mord3rca/gamma-launcher
//!
//! - [`spec`]    — static description of the whole CLI (single source of truth)
//! - [`mapper`]  — `(command, options)` → argv, with validation
//! - [`commands`] — the Tauri commands the frontend calls

pub mod commands;
mod mapper;
mod spec;

pub use commands::ActiveRun;

/// Sidecar name as configured in `tauri.conf.json > bundle.externalBin`
/// and allowed in `capabilities/default.json`.
pub(crate) const SIDECAR: &str = "gamma-launcher-v3.1";
