use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: String,
    pub build_cmd: String,
}

pub fn load_config(path: &str) -> Option<ProjectConfig> {
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_config(path: &str, config: &ProjectConfig) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(config)?;
    fs::write(path, data)
}
