use std::ops::{Deref, DerefMut};

use crate::get_window;

use wasm_bindgen::prelude::*;
use web_sys::Storage;

/// ### Web Storage API
/// 
/// The Storage interface of the Web Storage API provides access to a particular domain's 
/// session or local storage. It allows, for example, the addition, modification, or deletion of 
/// stored data items. To manipulate, for instance, the session storage for a domain, a call to 
/// Window.sessionStorage is made; whereas for local storage the call is made to Window.localStorage.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Storage
#[wasm_bindgen]
pub struct WebStorage {
    storage_type: WebStorageType,
    storage: Storage,
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum WebStorageType {
    LocalStorage,
    SessionStorage,
}

#[wasm_bindgen]
impl WebStorage {
    pub fn new(storage_type: WebStorageType) -> Result<WebStorage, JsValue> {
        let window = get_window()?;
        let storage = match storage_type {
            WebStorageType::LocalStorage => window.local_storage()?,
            WebStorageType::SessionStorage => window.session_storage()?,
        };

        match storage {
            Some(storage) => Ok(Self {
                storage_type: storage_type,
                storage,
            }),
            None => Err(JsValue::from_str("No storage available.."))
        }
    }

    pub fn storage_type(&self) -> WebStorageType {
        self.storage_type.clone()
    }
}

impl Deref for WebStorage {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for WebStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}