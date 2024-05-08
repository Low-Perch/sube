use std::collections::HashMap;
use futures_util::lock::Mutex;
use serde::{Serialize, Deserialize};

pub mod sites;

use sites::Site;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Persona {
    sites: Vec<Site>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personas {
    personas: HashMap::<String, Persona>
}

pub struct PersonasState(pub Mutex<Personas>);

impl Personas {
    pub fn new() -> Self {
        let personas = Self::get_personas().personas;
        Self { personas }
    }

    pub fn get_persona(&self, id: &str) -> Persona {
        self.personas.get(id).unwrap().to_owned()
    }

    pub fn get_personas() -> Self {
        if let Some(personas) = Self::read_personas() {
            personas
        } else {
            let personas = Self::get_default_personas();
            Self::write_to_personas(&personas);
            personas
        }
    }

    fn read_personas() -> Option<Self> {
        let config_dir = match Self::get_config_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("Error getting config directory: {}", err);
                return None;
            }
        };

        let personas_file_path = std::path::Path::new(&config_dir).join("personas.yml");

        if !personas_file_path.exists() {
            return None;
        }

        let personas_contents = Self::read_personas_file(personas_file_path.to_string_lossy().as_ref());

        Some(Self::get_yaml_content(&personas_contents))
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

    pub fn get_yaml_content(personas_content: &str) -> Self {
        match serde_yaml::from_str(personas_content) {
            Ok(contents) => {
                Self { ..contents }
            }
            Err(_) => {
                Self::get_default_personas()
            },
        }
    }

    pub fn read_personas_file(file: &str) -> String {
        match std::fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(_) => String::new(),
        }
    }

    pub fn write_to_personas(data: &Personas) {
        let config_dir = match Self::get_config_dir() {
            Ok(folder) => folder,
            Err(err) => {
                eprintln!("Error getting config directory: {:?}", err);
                return;
            }
        };

        let file_path = std::path::Path::new(&config_dir).join("personas.yml");

        if !file_path.exists() {
            if let Err(e) = std::fs::File::create(&file_path) {
                eprintln!("Error creating config file: {}", e);
                return;
            }
        }

        match serde_yaml::to_string(&data) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&file_path, content) {
                    eprintln!("Error writing to persona file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error serializing persona data to yaml: {}", e);
            }
        }
    }

    fn get_default_personas() -> Self {
        let personas = include_str!("personas.yml");
        Self::get_yaml_content(personas)
    }
}
