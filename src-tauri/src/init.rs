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

pub fn window_events(window: &Window, event: &WindowEvent) {
    let panel = window.get_webview("panel").unwrap();
    let portal = window.get_webview("portal").unwrap();

    match event {
        WindowEvent::Resized(dimensions) => {
            panel.set_size(LogicalSize { width: 36., height: dimensions.height.into() }).unwrap();
            portal.set_size(LogicalSize { width: dimensions.width as f64 - 36., height: dimensions.height as f64 }).unwrap();
        }
        WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size, .. } => {
            panel.set_size(LogicalSize { 
                width: 36., 
                height: (new_inner_size.height as f64 / scale_factor) as f64
            }).unwrap();
            portal.set_size(LogicalSize { 
                width: new_inner_size.width as f64 / scale_factor - 36.,
                height: new_inner_size.height  as f64 / scale_factor
            }).unwrap();
        }
        _ => {}
    };
}
