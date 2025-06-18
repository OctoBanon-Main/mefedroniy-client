use anyhow::{Context, Result};
use crate::config::{Config, file::{load_config, save_config}};

impl Config {
    pub fn load() -> Result<Self> {
        load_config()
    }

    pub fn save(&self) -> Result<()> {
        save_config(self)
    }

    pub fn add_server(&mut self, server: String) -> Result<()> {
        if self.servers.contains(&server) {
            anyhow::bail!("Server already added: {}", server);
        }

        self.servers.push(server);
        self.save().context("Failed to save configuration after adding the server")?;

        Ok(())
    }
}