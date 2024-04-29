use tauri::{Error as TauriError, LogicalSize, Manager, Url, Window};

use crate::init::{create_webview, PANEL_SIZE_POSITION};

#[tauri::command]
pub async fn set_webview_url(window: Window, url: String) -> Result<(), TauriError> {
    let mut portal = window.get_webview("portal").unwrap();
    if url.starts_with("https") {
        let portal_url = Url::parse(&url).unwrap();
        portal.navigate(portal_url);
    } else {
        portal.close()?;
        let window_size = window.inner_size()?;
        let width = window_size.width as f64;
        let height = window_size.height as f64;
        create_webview(
            &window,
            "portal",
            PANEL_SIZE_POSITION,
            LogicalSize::new(width - PANEL_SIZE_POSITION, height),
        )?;
    }

    Ok(())
}
