use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::stdin, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    // server address
    pub server: Server,
    // database address and auth
    pub database: Database,
}

impl Config {
    pub fn from_file(name: &str) -> Result<Self> {
        let path = PathBuf::from(name);

        if path.exists() {
            let file = fs::read(&path)?;
            let config: Config = toml::from_slice(&file)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: Server::default(),
            database: Database::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Server {
    ip: String,
    port: u16,
}

impl Server {
    pub fn address(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 50000
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    address: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl Database {
    fn init(mut self) -> Self {
        if self.address == None {
            error!("The database address is empty.");
            info!("[address]");
            self.address = Some(user_input());
        }
        if self.username == None || self.password == None {
            info!("Does the db require user auth? Y/N");
            match user_input().trim().to_lowercase().as_str() {
                "y" => {
                    Self::set_auth(&mut self);
                    info!("Updated the auth info.");
                },
                "n" => {
                    info!("Auth info set to 'None'.");
                },
                _ => {
                    info!("Auth info set to 'None'.")
                },
            }
        }
        self
    }

    fn set_auth(&mut self) {
        info!("[username]");
        self.username = Some(user_input());
        info!("[password]");
        self.password = Some(user_input());
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            address: None,
            username: None,
            password: None,
        }.init()
    }
}
