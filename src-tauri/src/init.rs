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
        LogicalSize::new(36., height),
    )?;

    let _webview2 = window.add_child(
        tauri::webview::WebviewBuilder::new(
          "portal",
          WebviewUrl::External("https://github.com".parse().unwrap()),
        )
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15")
        .auto_resize(),
        LogicalPosition::new(36., 0.),
        LogicalSize::new(width - 36., height),
    )?;

    Ok(())
}
