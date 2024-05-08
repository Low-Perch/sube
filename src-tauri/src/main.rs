// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::lock::Mutex;

mod app;
mod cmd;
mod config;
mod persona;

use persona::{Personas, PersonasState};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(PersonasState(Mutex::new(Personas::new())))
        .setup(app::setup::init)
        .on_window_event(app::window_event::init)
        .invoke_handler(tauri::generate_handler![
            cmd::set_webview_url,
            cmd::update_history,
            cmd::get_persona,
            cmd::get_sites,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
