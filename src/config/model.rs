use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub servers: Vec<String>,
    pub username: String,
    pub update_interval: u64,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_server(&mut self, server: String) -> Result<()> {
        if self.servers.contains(&server) {
            anyhow::bail!("Server already exists");
        }

        self.servers.push(server);
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            username: String::new(),
            update_interval: 5,
        }
    }
    
}