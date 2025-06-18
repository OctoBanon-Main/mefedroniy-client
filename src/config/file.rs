use std::{fs, path::Path};

use anyhow::{Context, Result};
use crate::config::{Config, CONFIG_FILE};

pub fn load_config() -> Result<Config> {
    if !Path::new(CONFIG_FILE).exists() {
        let default_config = Config::default();
        save_config(&default_config)
            .with_context(|| format!("Не удалось создать файл конфигурации: {}", CONFIG_FILE))?;
        return Ok(default_config);
    }

    // Если файл есть, читаем его
    let content = fs::read_to_string(CONFIG_FILE)
        .with_context(|| format!("Не удалось прочитать файл конфигурации: {}", CONFIG_FILE))?;

    let config = serde_json::from_str(&content)
        .with_context(|| "Ошибка при парсинге JSON конфигурации")?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
        .with_context(|| "Ошибка при сериализации конфигурации в JSON")?;

    fs::write(CONFIG_FILE, content)
        .with_context(|| format!("Не удалось записать конфигурацию в файл: {}", CONFIG_FILE))?;

    Ok(())
}