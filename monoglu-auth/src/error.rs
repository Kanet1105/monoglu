use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
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
    EmptyTableError(String),
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
            Self::EmptyTableError(table) => {
                write!(f, "'{}' is empty.", table)
            }
        }
    }
}

impl Error for ConfigError {}

/// Errors are returned to the user and visible on the browser.
pub enum UserError {
    InternalError,
    AuthClientNotFound,
    CsrfTokenExpired,
}

impl Debug for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal Error"),
            Self::AuthClientNotFound => write!(f, "Oauth2 client not found."),
            Self::CsrfTokenExpired => write!(f, "CSRF token is expired."),
        }
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthClientNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CsrfTokenExpired => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
