mod gamma;
mod window;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(gamma::ActiveRun::default())
        .setup(|app| {
            if let Some(main) = app.get_webview_window("main") {
                // Created hidden (tauri.conf.json) so the resize doesn't flicker.
                if let Err(e) = window::fit_to_monitor(&main) {
                    eprintln!("could not size main window: {e}");
                }
                main.show()?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            gamma::commands::gamma_launcher_schema,
            gamma::commands::gamma_launcher_validate,
            gamma::commands::gamma_launcher_requirements,
            gamma::commands::gamma_launcher_run,
            gamma::commands::gamma_launcher_cancel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
