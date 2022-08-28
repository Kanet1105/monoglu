use wasm_bindgen::prelude::*;
use super::get_window;

pub struct Storage(web_sys::Storage);

impl Storage {
    pub fn new() {
        
    }
}

/// # Local Storage
/// 
/// Static local storage getter.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
pub(crate) fn get_local_storage() -> web_sys::Storage {
    get_window()
        .local_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Local Storage object is not available.")
}

/// # Session Storage
/// 
/// Static session storage getter.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
pub(crate) fn get_session_storage() -> web_sys::Storage {
    get_window()
        .session_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Session Storage object is not available.")
}