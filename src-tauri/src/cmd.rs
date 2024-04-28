use tauri::{Manager, Url, Window};

#[tauri::command]
pub async fn set_webview_url(window: Window, url: String) -> Result<(), ()> {
    let mut portal = window.get_webview("portal").unwrap();
    let portal_url = Url::parse(&url).unwrap();
    portal.navigate(portal_url.clone());

    Ok(())
}
