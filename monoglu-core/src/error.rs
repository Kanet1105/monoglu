use std::{
    error::Error,
    fmt::{Debug, Display},
    path::PathBuf,
};

/// Error related to "load_config" module.
/// 
/// ```
/// Pub enum ConfigError {
///     // "Config.toml" file does not exist at the current dir.
///     ConfigPathError(PathBuf),
/// 
///     // The field does not exist in the given key table.
///     // ConfigKeyError(Key, Table)
///     ConfigKeyError(String, String),
/// }
/// ```
pub enum ConfigError {
    ConfigPathError(PathBuf),
    ConfigKeyError(String, String),
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigPathError(path) => write!(
                f,
                "The config file does not exist at {}",
                path.to_str().unwrap()
            ),
            Self::ConfigKeyError(key, table) => write!(
                f,
                "['{}'] does not exist in the table ['{}']",
                key, table,
            )
        }
    }
}

impl Error for ConfigError {}