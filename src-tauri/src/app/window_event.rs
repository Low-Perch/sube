use tauri::{LogicalSize, Manager, Window, WindowEvent};
use crate::config::Config;

const PANEL: &str = "panel";
const PORTAL: &str = "portal";

pub fn init(window: &Window, event: &WindowEvent) {
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
                    width: panel_size,
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

