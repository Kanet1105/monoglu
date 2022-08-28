use wasm_bindgen::prelude::*;
use super::window;

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
                let local_storage = local_storage();

                Self(local_storage)
            },
            StorageType::Session => {
                let session_storage = session_storage();

                Self(session_storage)
            }
        }
    }
}