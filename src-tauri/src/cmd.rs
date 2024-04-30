use tauri::{Error as TauriError, LogicalSize, Manager, Url, Window};

use crate::app::setup::{create_webview, PORTAL};
use crate::config::Config;

#[tauri::command]
pub async fn set_webview_url(window: Window, url: String) -> Result<(), TauriError> {
    let mut portal = window.get_webview(PORTAL).unwrap();
    if url.starts_with("https") {
        let portal_url = Url::parse(&url).unwrap();
        portal.navigate(portal_url);
    } else {
        portal.close()?;

        let panel_size = Config::get_config().window.panel_size;
        let window_size = window.inner_size()?;
        let width = window_size.width as f64;
        let height = window_size.height as f64;

        create_webview(
            &window,
            PORTAL,
            panel_size,
            LogicalSize::new(width - panel_size, height),
        )?;
    }

    Ok(())
}
