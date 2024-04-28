use std::error::Error;
use tauri::{App, Error as TauriError, LogicalPosition, LogicalSize, WebviewUrl, Url, Window, WindowEvent, Manager, Webview};

const WIDTH: f64 = 900.;
const HEIGHT: f64 = 600.;
const PANEL_SIZE_POSITION: f64 = 36.;

fn create_window(app: &App) -> Result<Window, TauriError> {
    tauri::window::WindowBuilder::new(app, "main")
        .inner_size(WIDTH, HEIGHT)
        .min_inner_size(WIDTH, HEIGHT)
        .build()
}

fn create_webview(window: &Window, label: &str, start_position: f64, width: f64) -> Result<Webview, TauriError> {
    let url = match label {
        "panel" => WebviewUrl::App(Default::default()),
        _ => WebviewUrl::External(Url::parse("https://google.com").unwrap())
    };

    window.add_child(
        tauri::webview::WebviewBuilder::new(label, url)
            // TODO: need to be able to select user_agent based on OS
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15"),
        LogicalPosition::new(start_position, 0.),
        LogicalSize::new(width, HEIGHT),
    )
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = create_window(app)?;
    create_webview(&window, "panel", 0., PANEL_SIZE_POSITION)?;
    create_webview(&window, "portal", 36., WIDTH - PANEL_SIZE_POSITION)?;

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
