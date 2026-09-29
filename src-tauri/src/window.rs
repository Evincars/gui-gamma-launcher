//! Sizes the main window to the monitor it opens on.

use tauri::{LogicalSize, Monitor, Runtime, WebviewWindow};

const PREFERRED: (f64, f64) = (1280.0, 900.0);
const MINIMUM: (f64, f64) = (860.0, 560.0);
/// Share of the screen the window may take, leaving room for panels/taskbars.
const SCREEN_FILL: f64 = 0.9;

fn monitor<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<Option<Monitor>> {
    if let Some(m) = window.current_monitor()? {
        return Ok(Some(m));
    }
    if let Some(m) = window.primary_monitor()? {
        return Ok(Some(m));
    }
    Ok(window.available_monitors()?.into_iter().next())
}

/// Use the preferred size, shrunk to fit the screen but never below the minimum.
pub fn fit_to_monitor<R: Runtime>(window: &WebviewWindow<R>) -> tauri::Result<()> {
    window.set_min_size(Some(LogicalSize::new(MINIMUM.0, MINIMUM.1)))?;
    let Some(monitor) = monitor(window)? else {
        return Ok(());
    };
    let screen = monitor.size().to_logical::<f64>(monitor.scale_factor());
    let width = PREFERRED.0.min(screen.width * SCREEN_FILL).max(MINIMUM.0);
    let height = PREFERRED.1.min(screen.height * SCREEN_FILL).max(MINIMUM.1);
    window.set_size(LogicalSize::new(width, height))?;
    window.center()
}
