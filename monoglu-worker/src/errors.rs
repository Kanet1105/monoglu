use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub enum ConfigError {
    ParsingError,
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ParsingError => write!(f, "In 'Config.toml' -> [server] missing a field."),
        }
    }
}

impl Error for ConfigError {}
