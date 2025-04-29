use std::fs;
use crate::config::{Config, CONFIG_FILE};

pub fn load_config() -> Config {
    fs::read_to_string(CONFIG_FILE)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| Config::new())
}

pub fn save_config(config: &Config) {
    serde_json::to_string_pretty(config)
        .map(|content| fs::write(CONFIG_FILE, content))
        .ok();
}