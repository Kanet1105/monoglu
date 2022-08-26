use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use crate::get_window;

use wasm_bindgen::prelude::*;

/// ## Web Storage API (!thread-safe)
/// 
/// The Web Storage API provides mechanisms by which browsers can store key/value pairs, 
/// in a much more intuitive fashion than using cookies.
/// 
/// ### SessionStorage
/// 
/// sessionStorage maintains a separate storage area for each given origin that's available 
/// for the duration of the page session (as long as the browser is open, including page reloads 
/// and restores). 
/// 
/// - Stores data only for a session, meaning that the data is stored until the browser (or tab) is closed.
/// - Data is never transferred to the server. 
/// - Storage limit is larger than a cookie (at most 5MB).
/// 
/// ### LocalStorage
/// 
/// localStorage does the same thing, but persists even when the browser is closed and reopened.
/// 
/// - Stores data with no expiration date, and gets cleared only through JavaScript, or clearing the 
///   Browser cache / Locally Stored Data. 
/// - Storage limit is the maximum amongst the two.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API
#[wasm_bindgen]
pub struct Storage(web_sys::Storage);

/// Pass the storage type as an argument when using a storage.
#[wasm_bindgen]
pub enum StorageType {
    Local,
    Session,
}

#[wasm_bindgen]
impl Storage {
    pub fn new(storage_type: StorageType) -> Result<Storage, JsValue> {
        let window = get_window()?;
        let storage = match storage_type {
            StorageType::Local => window.local_storage()?,
            StorageType::Session => window.session_storage()?,
        };

        match storage {
            Some(storage) => {
                Ok(Self(storage))
            }
            None => Err(JsValue::from_str("No storage available.."))
        }
    }

    /// The getItem() method of the Storage interface, when passed a key name, 
    /// will return that key's value, or null if the key does not exist, in the given Storage object.
    pub fn get_item(&self, key: &str) -> Result<String, JsValue> {
        match self.0.get_item(key) {
            Ok(value) => {
                match value {
                    Some(value) => Ok(value),
                    None => Err(JsValue::from_str("no key"))
                }
            },
            Err(value) => Err(value),
        }
    }

    /// The setItem() method of the Storage interface, when passed a key name and value, 
    /// will add that key to the given Storage object, or update that key's value if it already exists.
    pub fn set_item(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.0.set_item(key, value)
    }

    /// The removeItem() method of the Storage interface, when passed a key name, will remove 
    /// that key from the given Storage object if it exists. The Storage interface of the Web Storage API 
    /// provides access to a particular domain's session or local storage. If there is no item associated 
    /// with the given key, this method will do nothing.
    pub fn remove_item(&self, key: &str) -> Result<(), JsValue> {
        self.0.remove_item(key)
    }

    /// The clear() method of the Storage interface clears all keys stored in a given Storage object.
    pub fn clear(&self) -> Result<(), JsValue> {
        self.0.clear()
    }
}