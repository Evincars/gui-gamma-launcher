//! GUI wrapper for https://github.com/Mord3rca/gamma-launcher
//!
//! - [`spec`]     — static description of the whole CLI (single source of truth)
//! - [`validate`] — per-value checks (paths, tags, repo names)
//! - [`mapper`]   — `(command, options)` → argv
//! - [`schema`]   — spec → JSON for the UI
//! - [`sidecar`]  — how the bundled binary is invoked
//! - [`runner`]   — process lifecycle & output streaming
//! - [`commands`] — the Tauri commands the frontend calls

pub mod commands;
mod mapper;
mod runner;
mod schema;
mod sidecar;
mod spec;
mod validate;

pub use runner::ActiveRun;
