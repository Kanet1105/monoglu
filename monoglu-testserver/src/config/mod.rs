mod server;
mod tls;

use crate::prelude::*;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub struct Configuration {
    pub server: server::Server,
    pub tls: tls::TLS,
}

impl Configuration {
    pub fn from_file(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);

        if path.exists() {
            let file = fs::read(&path)?;
            let config: Configuration = toml::from_slice(&file)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            server: server::Server::default(),
            tls: tls::TLS::default(),
        }
    }
}
