use super::window;

use wasm_bindgen::prelude::*;

/// # Local Storage
/// 
/// Static local storage getter.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
pub(crate) fn local_storage() -> web_sys::Storage {
    window()
        .local_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Local Storage object is not available.")
}

/// # Session Storage
/// 
/// Static session storage getter.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
pub(crate) fn session_storage() -> web_sys::Storage {
    window()
        .session_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Session Storage object is not available.")
}

#[wasm_bindgen]
pub struct Storage(web_sys::Storage);

#[wasm_bindgen]
pub enum StorageType {
    Local,
    Session,
}

#[wasm_bindgen]
impl Storage {
    pub fn new(storage_type: StorageType) -> Self {
        match storage_type {
            StorageType::Local => {
                Self(local_storage())
            },
            StorageType::Session => {
                Self(session_storage())
            }
        }
    }

    pub fn get_item(&self, key: &str) -> Option<String> {
        match self.0.get_item(key) {
            Ok(value) => value.clone(),
            Err(js_value) => {
                log::error!("{:?}", js_value);
                panic!("Error wnwrapping Storage::['get_item()']");
            },
        }
    }

    pub fn set_item(&self, key: &str, value: &str) {
        match self.0.set_item(key, value) {
            Ok(()) => {},
            Err(js_value) => {
                log::error!("{:?}", js_value);
                panic!("Error wnwrapping Storage::['get_item()']");
            }
        }
    }
}