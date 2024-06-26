use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Error as TauriError, LogicalPosition, LogicalSize, Manager, State, Window};

use crate::app::setup::{create_webview, PORTAL};
use crate::config::Config;
use crate::persona::sites::Site;
use crate::persona::{Personas, PersonasState};

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
pub async fn get_sites(personas: State<'_, PersonasState>) -> Result<Vec<Site>, TauriError> {
    let persona_guard = personas.0.lock().await;
    let config = Config::get_config();

    let persona = persona_guard.get_persona(&config.persona);
    Ok(persona.sites)
}

#[tauri::command]
pub async fn get_persona() -> String {
    let config = Config::get_config();
    config.persona
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    profile: String,
    profiles: Vec<String>,
    personas: Option<Personas>,
}

#[tauri::command]
pub async fn get_personas(personas: State<'_, PersonasState>) -> Result<Profile, TauriError> {
    let persona_guard = personas.0.lock().await;
    let profile = get_persona().await;
    let profiles = persona_guard.get_personas_keys();

    Ok(Profile {
        profiles,
        profile,
        personas: None,
    })
}

#[tauri::command]
pub async fn init(personas: State<'_, PersonasState>) -> Result<Profile, TauriError> {
    let persona_guard = personas.0.lock().await;
    let personas = &persona_guard.personas;
    let profiles = persona_guard.get_personas_keys();
    let profile = get_persona().await;

    Ok(Profile {
        profiles,
        profile,
        personas: Some(Personas {
            personas: personas.clone(),
        }),
    })
}

#[tauri::command]
pub fn update_persona(app: AppHandle, persona: &str) {
    let persona = Config::update_persona(persona);
    app.emit("new_persona", persona).unwrap();
}
