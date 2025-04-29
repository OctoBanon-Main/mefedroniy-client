use crate::config::{Config, file::save_config};

impl Config {
    pub fn load() -> Self {
        super::file::load_config()
    }
    
    pub fn save(&self) {
        save_config(self)
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