// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(commands::ServerState(std::sync::Mutex::new(None)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::setup_server_files,
            commands::create_server_runner,
            commands::create_eula,
            commands::start_server,
            commands::stop_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}