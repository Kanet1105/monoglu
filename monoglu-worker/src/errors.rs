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

impl std::error::Error for ConfigError {}

#[test]
fn print_error() {
    println!("{:?}", ConfigError::ParsingError);
}

#[test]
fn fire_drill() {
    fire(true).unwrap();
}

fn fire(is_err: bool) -> Result<(), Box<dyn std::error::Error>> {
    match is_err {
        true => Err(ConfigError::ParsingError.into()),
        false => Ok(()),
    }
}
