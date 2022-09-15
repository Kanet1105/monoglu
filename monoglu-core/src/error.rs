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
///
///     // The field does not exist in the given key table.
///     // KeyError(key, table)
///     KeyError(String, String),
///
///     // The table is empty.
///     // EmptyTable(table)
///     EmptyTable(String)
/// }
/// ```
pub enum ConfigError {
    PathError(PathBuf),
    KeyError(String, String),
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
            Self::KeyError(key, table) => {
                write!(f, "['{}'] does not exist in the table ['{}']", key, table)
            }
            Self::EmptyTable(table) => {
                write!(f, "['{}'] table is empty.", table)
            }
        }
    }
}

impl Error for ConfigError {}
