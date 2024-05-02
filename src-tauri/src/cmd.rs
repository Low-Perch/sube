use tauri::{Error as TauriError, LogicalPosition, LogicalSize, Manager, Window};

use crate::app::setup::{create_webview, PORTAL};
use crate::config::Config;

#[tauri::command]
pub async fn set_webview_url(window: Window, url: String) -> Result<(), TauriError> {
    let portal = window.get_webview(PORTAL).unwrap();
    portal.close()?;

    let panel_size = Config::get_config().window.panel_size;
    let window_size = window.inner_size()?;
    let scale_f = window.scale_factor()?;
    let width = window_size.width as f64;
    let height = window_size.height as f64;

    create_webview(
        &window,
        PORTAL,
        LogicalPosition::new(panel_size, panel_size),
        LogicalSize::new(width / scale_f - panel_size, height / scale_f),
        Some(&url),
    )?;

    Ok(())
}
