use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
    pub panel_size: f64,
    pub resizable: bool,
}
