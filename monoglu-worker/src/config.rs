use crate::prelude::*;

pub struct ConfigBuilder;

impl ConfigBuilder {
    pub fn new() -> Result<Config, Error> {
        let mut config_path = current_dir()?;
        config_path.push("Config.toml");

        let config = match config_path.exists() {
            true => ConfigBuilder::open_config(config_path)?,
            false => ConfigBuilder::default_settings(config_path)?,
        };
        Ok(config)
    }

    fn default_settings(config_path: PathBuf) -> Result<Config, Error> {
        let config = Config::default();
        let config_toml = toml::to_string(&config).unwrap();
        fs::write(config_path, config_toml)?;
        Ok(config)
    }

    fn open_config(config_path: PathBuf) -> Result<Config, Error> {
        let mut config_buffer = String::new();
        fs::File::open(config_path)?.read_to_string(&mut config_buffer)?;

        let config = toml::from_str(&config_buffer).with_context(|| ConfigError::ParsingError)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: Server,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: Server::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub ip: String,
    pub port: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".into(),
            port: "8080".into(),
        }
    }
}
