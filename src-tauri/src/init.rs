use std::error::Error;
use tauri::{LogicalPosition, LogicalSize, WebviewUrl};

pub fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn Error>> {
    let width = 1200.;
    let height = 800.;
    let window = tauri::window::WindowBuilder::new(app, "main")
    .inner_size(width, height)
    .min_inner_size(1200., 800.)
    .build()?;

    let _webview1 = window.add_child(
        tauri::webview::WebviewBuilder::new("panel", WebviewUrl::App(Default::default()))
          .auto_resize(),
        LogicalPosition::new(0., 0.),
        LogicalSize::new(25., height),
    )?;

    let _webview2 = window.add_child(
        tauri::webview::WebviewBuilder::new(
          "portal",
          WebviewUrl::External("https://github.com".parse().unwrap()),
        )
        .auto_resize(),
        LogicalPosition::new(25., 0.),
        LogicalSize::new(width - 25., height),
    )?;

    Ok(())
}
