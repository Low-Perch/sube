use std::error::Error;
use tauri::{
    App, Error as TauriError, LogicalPosition, LogicalSize, Url, Webview, WebviewUrl, Window,
};

use crate::app::shortcut;
use crate::config::Config;

pub const MAIN: &str = "main";
pub const PANEL: &str = "panel";
pub const PORTAL: &str = "portal";
pub const TITLE_BAR: &str = "title_bar";

fn create_window(app: &App, config: &Config) -> Result<Window, TauriError> {
    let window = &config.window;

    tauri::window::WindowBuilder::new(app, MAIN)
        .visible(false)
        .always_on_top(true)
        .resizable(window.resizable)
        .decorations(false)
        .visible_on_all_workspaces(true)
        .inner_size(window.width, window.height)
        .min_inner_size(window.width, window.height)
        .build()
}

pub fn create_webview(
    window: &Window,
    label: &str,
    position: LogicalPosition<f64>,
    size: LogicalSize<f64>,
    url: Option<&str>,
) -> Result<Webview, TauriError> {
    let url = match label {
        PANEL => WebviewUrl::App("../../panel.html".into()),
        PORTAL => {
            if let Some(url) = url {
                if url.starts_with("https") {
                    WebviewUrl::External(Url::parse(url).unwrap())
                } else {
                    WebviewUrl::App("../../portal.html".into())
                }
            } else {
                WebviewUrl::App("../../portal.html".into())
            }
        }
        _ => WebviewUrl::App("../../title_bar.html".into()),
    };

    let config = Config::get_config();
    let user_agent = config.user_agent.get();

    let script = match label {
        PORTAL => include_str!("inject.js"),
        _ => "",
    };

    window.add_child(
        tauri::webview::WebviewBuilder::new(label, url)
            .user_agent(user_agent)
            .initialization_script(script),
        position,
        size,
    )
}

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    // required for window.always_on_top.
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let config = Config::get_config();
    let window = create_window(app, &config)?;
    let window_cfg = &config.window;

    create_webview(
        &window,
        TITLE_BAR,
        LogicalPosition::new(0., 0.),
        LogicalSize::new(window_cfg.width, window_cfg.panel_size),
        None,
    )?;

    create_webview(
        &window,
        PANEL,
        LogicalPosition::new(0., window_cfg.panel_size),
        LogicalSize::new(window_cfg.width, window_cfg.height - window_cfg.panel_size),
        None,
    )?;

    create_webview(
        &window,
        PORTAL,
        LogicalPosition::new(window_cfg.panel_size, window_cfg.panel_size),
        LogicalSize::new(
            window_cfg.width - window_cfg.panel_size,
            window_cfg.height - window_cfg.panel_size,
        ),
        None,
    )?;

    tauri::async_runtime::spawn(async move {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        window.show().unwrap();
    });

    shortcut::init(app)?;

    Ok(())
}
