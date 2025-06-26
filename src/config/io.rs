use super::model::Config;
use std::{fs, path::Path};
use anyhow::{Context, Result};

const CONFIG_FILE: &str = "config.json";

pub fn load() -> Result<Config> {
    if !Path::new(CONFIG_FILE).exists() {
        let default_config = Config::default();
        save(&default_config)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(CONFIG_FILE)
        .with_context(|| format!("Failed to read configuration file: {}", CONFIG_FILE))?;

    serde_json::from_str(&content)
        .with_context(|| "Error parsing JSON configuration")
}

pub fn save(config: &Config) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
        .with_context(|| "Error serializing configuration to JSON")?;

    fs::write(CONFIG_FILE, content)
        .with_context(|| format!("Failed to write configuration to file: {}", CONFIG_FILE))
}