mod gamma;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(gamma::ActiveRun::default())
        .invoke_handler(tauri::generate_handler![
            gamma::commands::gamma_launcher_schema,
            gamma::commands::gamma_launcher_preview,
            gamma::commands::gamma_launcher_args,
            gamma::commands::gamma_launcher_version,
            gamma::commands::gamma_launcher_run,
            gamma::commands::gamma_launcher_cancel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
