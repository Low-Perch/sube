use serde::{Deserialize, Serialize};
use tauri::{Error as TauriError, LogicalPosition, LogicalSize, Manager, State, Window};

use crate::app::setup::{create_webview, PORTAL};
use crate::config::Config;
use crate::persona::{Persona, PersonasState};

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

#[derive(Serialize, Deserialize)]
pub enum History {
    Back,
    Forward,
    Reload,
}

#[tauri::command]
pub async fn update_history(window: Window, state: History) {
    let portal = window.get_webview(PORTAL).unwrap();
    let update = match state {
        History::Back => "window.history.back()",
        History::Forward => "window.history.forward()",
        History::Reload => "window.location.reload()",
    };

    portal.eval(update).unwrap();
}

#[tauri::command]
pub async fn get_persona(
    personas: State<'_, PersonasState>,
    id: Option<&str>,
) -> Result<Persona, TauriError> {
    let persona_guard = personas.0.lock().await;

    let id = id.unwrap_or("me");

    let persona = persona_guard.get_persona(id);
    Ok(persona)
}
