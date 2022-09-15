use crate::auth::ClientBuilder;
use crate::error::{ConfigError, Exception};
use crate::storage::StorageBuilder;
use config::Config;
use oauth2::basic::BasicClient;
use std::{collections::HashMap, path::PathBuf};
use tracing::info;

/// Read config once and for all at the beginning of building the state.
/// Once the Config is returned, pass the config as a reference to all
/// the other "load_" prefixed functions.
pub(crate) fn load_config() -> Result<Config, Exception> {
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
        false => Err(ConfigError::PathError(config_path).into()),
    }
}

pub(crate) fn load_storage(config: &Config) -> Result<sled::Db, Exception> {
    let table = "storage".to_string();
    let storage = config
        .get::<StorageBuilder>(&table)?
        .new()?;
    Ok(storage)
}

pub(crate) fn load_auth_client(config: &Config) -> Result<HashMap<String, BasicClient>, Exception> {
    let mut auth_clients = HashMap::<String, BasicClient>::new();
    let table = "oauth2".to_string();
    let oauth2 = config.get_table(&table)?;
    if oauth2.is_empty() {
        return Err(ConfigError::EmptyTable(table).into());
    }

    for (name, value) in oauth2 {
        let client = value
            .try_deserialize::<ClientBuilder>()?
            .new()?;
        auth_clients.insert(name, client);
    }
    Ok(auth_clients)
}
