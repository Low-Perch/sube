// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{LogicalPosition, LogicalSize, WebviewUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 800.;
            let height = 600.;
            let window = tauri::window::WindowBuilder::new(app, "main")
            .inner_size(width, height)
            .build()?;

            let _webview1 = window.add_child(
                tauri::webview::WebviewBuilder::new("main1", WebviewUrl::App(Default::default()))
                  .auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width * 0.05, height),
            )?;
            let _webview2 = window.add_child(
                tauri::webview::WebviewBuilder::new(
                  "main2",
                  WebviewUrl::External("https://github.com".parse().unwrap()),
                )
                .auto_resize(),
                LogicalPosition::new(width * 0.05, 0.),
                LogicalSize::new(width * 0.95, height),
            )?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
