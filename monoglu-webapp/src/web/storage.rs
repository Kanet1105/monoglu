use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebStorage(web_sys::Storage);

#[wasm_bindgen]
pub enum WebStorageType {
    Local,
    Session,
}

#[wasm_bindgen]
impl WebStorage {
    pub fn new(window: &web_sys::Window, storage_type: WebStorageType) -> Self {
        match storage_type {
            WebStorageType::Local => {
                let local = window.local_storage()
                    .expect_throw("Error unwrapping ['local_storage()'].")
                    .expect_throw("Local Storage is unavailable.");
                Self(local)
            },
            WebStorageType::Session => {
                let session = window.session_storage()
                    .expect_throw("Error unwrapping ['local_storage()'].")
                    .expect_throw("session Storage is unavailable.");
                Self(session)
            },
        }
    }

    pub fn get_item(&self, key: &str) -> Option<String> {
        match self.0.get_item(key) {
            Ok(value) => value.clone(),
            Err(error) => {
                log::error!("{:?}", error);
                panic!("Error wnwrapping web_sys::Storage::['get_item()']");
            },
        }
    }

    pub fn set_item(&self, key: &str, value: &str) {
        match self.0.set_item(key, value) {
            Ok(()) => {},
            Err(error) => {
                log::error!("{:?}", error);
                panic!("Error wnwrapping web_sys::Storage::['set_item()']");
            }
        }
    }
}