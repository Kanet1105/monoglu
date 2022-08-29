mod fetch;
mod performance;
mod storage;
mod window;

pub(crate) use window::window;
pub use storage::{Storage, StorageType}; 

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

/// # Web API
/// 
/// When writing code for the Web, there are a large number of 
/// Web APIs available. Below is a list of all the APIs and 
/// interfaces (object types) that you may be able to use while 
/// developing your Web app or site.
/// 
/// ['WebAPI'] module provides the access to the window object and 
/// both local and session storage object wrapper to the caller.
pub struct WebAPI {
    pub window: web_sys::Window,
    pub local_storage: Storage,
    pub session_storage: Storage,
}

impl WebAPI {
    /// Subclass the Web API object that needs to be configured properly or
    /// requires nested error handlings like <Option<T>, JsValue>.
    pub fn new() -> Self {
        Self {
            window: window(),
            local_storage: Storage::new(StorageType::Local),
            session_storage: Storage::new(StorageType::Session),
        }
    }

    /// ['WebAPI::spawn_task()'] runs the Rust Future on the current thread.
    pub fn spawn_task<F>(&self, function: F) 
    where
        F: FnOnce() + 'static
    {
        spawn_local(async move {
            function();
        });
    }

    pub fn fetch(&self, url: &str) {
        
    }
}