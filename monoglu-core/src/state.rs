use crate::error::StateError;
use crate::Exception;
use config::{self, Config};
use sled;
use std::{
    collections::HashMap,
    fs::File,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};
use tracing::info;

pub struct SharedState(Arc<InnerState>);

impl SharedState {
    pub fn new() -> Result<Self, Exception> {
        let inner_state = InnerState::new()?;
        Ok(Self(Arc::new(inner_state)))
    }
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for SharedState {
    type Target = Arc<InnerState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct InnerState {
    
}

impl InnerState {
    pub fn new() -> Result<Self, Exception> {
        let config = load_config()?;
        let storage = load_storage(&config)?;

        Ok(Self {
            
        })
    }
}

/// Read config once and for all at the beginning of building the state.
/// Once the Config is returned, pass the config as a reference to all
/// the other "load_" prefixed functions.
fn load_config() -> Result<Config, Exception> {
    let mut config_path = std::env::current_dir()?;
    config_path.push("Config.toml");

    match config_path.exists() {
        true => {
            let config = Config::builder()
                .add_source(config::File::from(config_path.as_ref()))
                .build()?;
            info!("Load configurations at {}", config_path.display());
            Ok(config)
        }
        false => Err(StateError::ConfigPathError(config_path).into()),
    }
}

fn load_storage(config: &Config) -> Result<sled::Db, Exception> {
    let table = "persistence".to_string();
    let key = "path".to_string();
    let config_map = config.get_table(&table)?;

    let storage_path = match config_map.get(&key) {
        Some(value) => {
            let path: PathBuf = value
                .clone()
                .into_array()?
                .iter()
                .map(|x| x.to_string())
                .collect();
            path
        },
        None => return Err(StateError::ConfigKeyError(key, table).into()),
    };
    let storage = sled::open(storage_path)?;
    if storage.was_recovered() {
        info!("The database was recovered from the last process.");
    }
    Ok(storage)
}

fn load_auth_client(config: &Config) -> Result<(), Exception> {
    Ok(())
}