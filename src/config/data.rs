use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub servers: Vec<String>,
    pub username: String,
    pub update_interval: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            username: String::new(),
            update_interval: 5,
        }
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