mod performance;
mod storage;
mod window;

pub(crate) use window::window;
pub use storage::{Storage, StorageType};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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

    /// # Using the Fetch API 
    /// 
    /// The Fetch API provides a JavaScript interface for accessing and 
    /// manipulating parts of the HTTP pipeline, such as requests and responses. 
    /// It also provides a global fetch() method that provides an easy, 
    /// logical way to fetch resources asynchronously across the network.
    /// 
    /// This kind of functionality was previously achieved using XMLHttpRequest. 
    /// Fetch provides a better alternative that can be easily used by other 
    /// technologies such as Service Workers. Fetch also provides a single 
    /// logical place to define other HTTP-related concepts such as CORS and 
    /// extensions to HTTP.
    pub fn fetch(&self, url: &str, method: Method, mode: Mode) -> JsFuture {
        // build a header with given method and mode arguments.
        let mut options = web_sys::RequestInit::new();

        match method {
            Method::Get => options.method("GET"),
            Method::Post => options.method("POST"),
        };

        match mode {
            Mode::CORS => options.mode(web_sys::RequestMode::Cors),
            Mode::SameOrigin => options.mode(web_sys::RequestMode::SameOrigin),
        };

        // build request with the given optional parameters.
        let new_request = match web_sys::Request::new_with_str_and_init(url, &options) {
            Ok(request) => request,
            Err(js_value) => {
                log::error!("{:?}", js_value);
                panic!("Error unwrapping ['web_sys::Request::new_with_str_and_init()']");
            }
        };

        JsFuture::from(self.window.fetch_with_request(&new_request))
    }

    pub fn task_spawn(&self, task: JsFuture) {
        spawn_local(async move {
            log::debug!("{:?}", task);
        });
    }
}

pub enum Method {
    Get,
    Post,
}

pub enum Mode {
    CORS,
    SameOrigin,
}