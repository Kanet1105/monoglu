use std::{
    error::Error,
    fmt::{Debug, Display},
    path::PathBuf,
};

pub type Exception = Box<dyn std::error::Error>;

/// Errors related to "config.rs" module.
///
/// ```
/// Pub enum ConfigError {
///     // "Config.toml" file does not exist in the current dir.
///     PathError(PathBuf),
///     EmptyTable(String)
/// }
/// ```
pub enum ConfigError {
    PathError(PathBuf),
    EmptyTable(String),
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathError(path) => {
                write!(
                    f,
                    "The config file does not exist at {}",
                    path.to_str().unwrap()
                )
            }
            Self::EmptyTable(table) => {
                write!(f, "'{}' is empty.", table)
            }
        }
    }
}

impl Error for ConfigError {}
