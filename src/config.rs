use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    net::ToSocketAddrs,
};

const CONFIG_FILE: &str = "servers.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub servers: Vec<String>,
    pub username: String,
    pub update_interval: u64,
}

impl Config {
    pub fn load() -> Self {
        fs::read_to_string(CONFIG_FILE)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_else(|| Self {
                servers: Vec::new(),
                username: String::new(),
                update_interval: 5,
            })
    }
    
    pub fn save(&self) {
        serde_json::to_string_pretty(self)
            .map(|content| fs::write(CONFIG_FILE, content))
            .ok();
    }

    pub fn add_server(&mut self, server: String) {
        match self.servers.contains(&server) {
            false => {
                self.servers.push(server);
                self.save();
            }
            true => eprintln!("Ошибка: сервер уже добавлен!"),
        }
    }
}

pub fn read_input(prompt: &str) -> io::Result<String> {
    print!("\x1B[32m{}\x1B[0m", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub async fn resolve_address(address: &str, port: &str) -> Option<String> {
    let addr = format!("{}:{}", address, port);
    addr.to_socket_addrs()
        .ok()
        .and_then(|mut iter| iter.next())
        .map(|a| a.to_string())
}