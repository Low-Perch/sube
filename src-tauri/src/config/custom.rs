use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
    pub scripts: Vec<String>,
}
