// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod config;
mod init;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(init::setup)
        .on_window_event(init::window_events)
        .invoke_handler(tauri::generate_handler![cmd::set_webview_url,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
