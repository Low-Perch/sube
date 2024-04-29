use std::error::Error;
use tauri::{
    App, Error as TauriError, LogicalPosition, LogicalSize, Manager, Webview, WebviewUrl, Window,
    WindowEvent,
};

use crate::config::Config;

pub const PANEL: &str = "panel";
pub const PORTAL: &str = "portal";

fn create_window(app: &App, config: &Config) -> Result<Window, TauriError> {
    let window = &config.window;

    tauri::window::WindowBuilder::new(app, "main")
        .inner_size(window.width, window.height)
        .min_inner_size(window.width, window.height)
        .visible(false)
        .resizable(window.resizable)
        .build()
}

pub fn create_webview(
    window: &Window,
    label: &str,
    start_position: f64,
    size: LogicalSize<f64>,
) -> Result<Webview, TauriError> {
    let url = match label {
        "panel" => WebviewUrl::App("../../panel.html".into()),
        _ => WebviewUrl::App("../../portal.html".into()),
    };

    let position = LogicalPosition::new(start_position, 0.);
    let config = Config::get_config();
    let user_agent = config.user_agent.get();

    window.add_child(
        tauri::webview::WebviewBuilder::new(label, url)
        .user_agent(user_agent),
        position,
        size
    )
}

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let config = Config::get_config();
    let window = create_window(app, &config)?;

    let window_cfg = &config.window; 
    create_webview(
        &window,
        PANEL,
        0.,
        LogicalSize::new(window_cfg.width, window_cfg.height),
    )?;
    create_webview(
        &window,
        PORTAL,
        window_cfg.panel_size,
        LogicalSize::new(window_cfg.width - window_cfg.panel_size, window_cfg.height),
    )?;

    tauri::async_runtime::spawn(async move {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        window.show().unwrap();
    });

    Ok(())
}

pub fn window_events(window: &Window, event: &WindowEvent) {
    let panel = window.get_webview(PANEL).unwrap();
    let portal = window.get_webview(PORTAL).unwrap();

    let panel_size = Config::get_config().window.panel_size;

    match event {
        WindowEvent::Resized(dimensions) => {
            panel
                .set_size(LogicalSize {
                    width: panel_size,
                    height: dimensions.height.into(),
                })
                .unwrap();
            portal
                .set_size(LogicalSize {
                    width: dimensions.width as f64 - panel_size,
                    height: dimensions.height as f64,
                })
                .unwrap();
        }
        WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
            ..
        } => {
            panel
                .set_size(LogicalSize {
                    width:  panel_size,
                    height: (new_inner_size.height as f64 / scale_factor),
                })
                .unwrap();
            portal
                .set_size(LogicalSize {
                    width: new_inner_size.width as f64 / scale_factor - panel_size,
                    height: new_inner_size.height as f64 / scale_factor,
                })
                .unwrap();
        }
        _ => {}
    };
}
