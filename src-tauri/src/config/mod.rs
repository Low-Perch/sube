use serde::Deserialize;

pub mod window;
pub mod user_agent;

use window::WindowConfig;
use user_agent::UserAgent;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub scripts: Vec<String>,
    pub window: WindowConfig,
    pub user_agent: UserAgent,
}

impl Config {
    pub fn get_config() -> Config {
        let config: Config = serde_json::from_str(include_str!("sube.json"))
            .expect("Failed to parse sube config");
        config
    }
}
