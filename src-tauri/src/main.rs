// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cmd;
mod config;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(app::setup::init)
        .on_window_event(app::window_event::init)
        .invoke_handler(tauri::generate_handler![
            cmd::set_webview_url,
            cmd::update_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
