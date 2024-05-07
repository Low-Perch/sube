use serde::{Deserialize, Serialize};

pub mod custom;
pub mod user_agent;
pub mod window;

use custom::Custom;
use user_agent::UserAgent;
use window::WindowConfig;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub custom: Custom,
    pub window: WindowConfig,
    pub user_agent: UserAgent,
}

impl Config {
    pub fn get_config() -> Self {
        // Try to read config from user config
        if let Some(config) = Self::read_config() {
            config
        } else {
            // Config DNE or unreadable, read from sube.toml
            let default_config = Self::get_default_config();
            Self::write_to_config(&default_config);
            default_config
        }
    }

    fn read_config() -> Option<Self> {
        let config_dir = match Self::get_config_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("Error getting config directory: {}", err);
                return None;
            }
        };

        let config_file_path = std::path::Path::new(&config_dir).join("config.toml");

        if !config_file_path.exists() {
            return None;
        }

        let config_contents = Self::read_config_file(config_file_path.to_string_lossy().as_ref());

        Some(Self::get_toml_content(&config_contents))
    }

    fn get_config_dir() -> Result<String, String> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".sube");

            if !config_dir.exists() {
                if let Err(err) = std::fs::create_dir_all(&config_dir) {
                    return Err(format!("Failed to create config folder: {}", err));
                }
            }

            Ok(config_dir.to_string_lossy().into_owned())
        } else {
            Err("No home directory".to_string())
        }
    }

    pub fn get_toml_content(config_contents: &str) -> Self {
        match toml::from_str(config_contents) {
            Ok(contents) => Self { ..contents },
            Err(_) => Self {
                ..Self::get_default_config()
            },
        }
    }

    pub fn read_config_file(file: &str) -> String {
        match std::fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(_) => String::new(),
        }
    }

    pub fn write_to_config(data: &Config) {
        let config_dir = match Self::get_config_dir() {
            Ok(folder) => folder,
            Err(err) => {
                eprintln!("Error getting config directory: {:?}", err);
                return;
            }
        };

        let file_path = std::path::Path::new(&config_dir).join("config.toml");

        if !file_path.exists() {
            if let Err(e) = std::fs::File::create(&file_path) {
                eprintln!("Error creating config file: {}", e);
                return;
            }
        }

        match toml::to_string(&data) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&file_path, content) {
                    eprintln!("Error writing to config file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error serializing config data to toml: {}", e);
            }
        }
    }

    fn get_default_config() -> Self {
        let default_config = include_str!("sube.toml");
        Self::get_toml_content(default_config)
    }
}
