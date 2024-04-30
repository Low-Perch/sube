use crate::config::Config;
use tauri::{LogicalSize, Manager, PhysicalSize, Window, WindowEvent};

const PANEL: &str = "panel";
const PORTAL: &str = "portal";

fn resize_webviews(window: &Window, dimensions: PhysicalSize<u32>, scale_f: f64) {
    let panel = window.get_webview(PANEL).unwrap();
    let portal = window.get_webview(PORTAL).unwrap();
    let panel_size = Config::get_config().window.panel_size;

    panel
        .set_size(LogicalSize {
            width: panel_size,
            height: (dimensions.height as f64 / scale_f),
        })
        .unwrap();
    portal
        .set_size(LogicalSize {
            width: dimensions.width as f64 / scale_f - panel_size,
            height: dimensions.height as f64 / scale_f,
        })
        .unwrap();
}

pub fn init(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::Resized(dimensions) => {
            let scale_f = window.scale_factor().unwrap();
            resize_webviews(window, *dimensions, scale_f);
        }
        WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
            ..
        } => {
            resize_webviews(window, *new_inner_size, *scale_factor);
        }
        _ => {}
    };
}
