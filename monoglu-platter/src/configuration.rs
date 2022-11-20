use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    // server address
    pub ip: String,
    pub port: u16,
    // database address and auth
    pub database: String,
    pub db_id: String,
    pub db_password: String,
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
            ip: "127.0.0.1".to_string(),
            port: 50000,
            database: "127.0.0.1:8086".to_string(),
            db_id: "kanet".to_string(),
            db_password: "01076737627".to_string(),
        }
    }
}
