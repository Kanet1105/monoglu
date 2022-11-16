use crate::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    ops::Deref,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    #[serde(skip)]
    pub init_time: Clock,
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

    pub fn up_time(&self) -> Duration {
        self.init_time.elapsed()
    }

    pub fn join_key(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update("todo(): Add hashable params..");
        let result = hasher.finalize();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 50000,
            init_time: Clock::default(),
        }
    }
}

/// Clock struct wraps [std::time::Instant] to implement 
/// [std::default::Default].
pub struct Clock(Instant);

impl Default for Clock {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl Deref for Clock {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
