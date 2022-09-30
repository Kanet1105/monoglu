use std::{
    error::Error,
    fmt::{Debug, Display},
};

fn window_handle() -> Result<web_sys::Window, WebAPIError> {
    match web_sys::window() {
        Some(window) => Ok(window),
        None => Err(WebAPIError::WindowUnavailable),
    }
}

pub fn local_storage() -> Result<web_sys::Storage, WebAPIError> {
    let window = window_handle()?;
    match window.local_storage() {
        Ok(storage_result) => match storage_result {
            Some(storage) => Ok(storage),
            None => Err(WebAPIError::LocalStorageNotFound),
        },
        Err(js_value) => {
            log::error!("{:?}", js_value);
            Err(WebAPIError::LocalStorageNotFound)
        }
    }
}

pub fn session_storage() -> Result<web_sys::Storage, WebAPIError> {
    let window = window_handle()?;
    match window.session_storage() {
        Ok(storage_result) => match storage_result {
            Some(storage) => Ok(storage),
            None => Err(WebAPIError::SessionStorageNotFound),
        },
        Err(js_value) => {
            log::error!("{:?}", js_value);
            Err(WebAPIError::SessionStorageNotFound)
        }
    }
}

pub enum WebAPIError {
    WindowUnavailable,
    LocalStorageNotFound,
    SessionStorageNotFound,
}

impl Debug for WebAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for WebAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowUnavailable => write!(f, "Failed to get the window handle."),
            Self::LocalStorageNotFound => write!(f, "Failed to get the local storage."),
            Self::SessionStorageNotFound => write!(f, "Failed to get the session storage."),
        }
    }
}

impl Error for WebAPIError {}
