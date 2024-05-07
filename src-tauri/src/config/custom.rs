use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
    pub scripts: Vec<String>,
}
