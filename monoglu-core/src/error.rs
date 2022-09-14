use std::{
    error::Error,
    fmt::{Debug, Display},
    path::PathBuf,
};
/// Error related to "state" module.
pub enum StateError {
    ConfigPathError(PathBuf),

}

impl Debug for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ConfigPathError(path) => write!(f, "The config file does not exist at {}", path),
        }
    }
}

impl Error for StateError {}
