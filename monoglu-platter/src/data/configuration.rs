use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
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
        }
    }
}
