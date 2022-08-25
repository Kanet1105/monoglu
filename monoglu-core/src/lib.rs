mod fetch;
mod log;
mod storage;

pub mod prelude {
    pub use crate::fetch::request;
    pub use crate::get_window;
    pub use crate::log::{log, log_raw};
    pub use crate::storage::{WebStorage, WebStorageType};

    pub use wasm_bindgen::prelude::*;
}

pub use crate::prelude::*;

#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    let local_storage = WebStorage::new(WebStorageType::LocalStorage)?;
    let session_storage = WebStorage::new(WebStorageType::SessionStorage)?;
    Ok(())
}

/// ### The Window Interface.
/// 
/// The Window interface represents a window containing a DOM document; 
/// the document property points to the DOM document loaded in that window.
///  
/// A window for a given document can be obtained using the document.defaultView property.
///  
/// A global variable, window, representing the window in which the script is running, 
/// is exposed to JavaScript code.
/// 
/// The Window interface is home to a variety of functions, namespaces, objects, and 
/// constructors which are not necessarily directly associated with the concept of 
/// a user interface window. However, the Window interface is a suitable place to include 
/// these items that need to be globally available. Many of these are documented in the 
/// JavaScript Reference and the DOM Reference.
/// 
/// In a tabbed browser, each tab is represented by its own Window object; the global window
/// seen by JavaScript code running within a given tab always represents the tab in which 
/// the code is running. That said, even in a tabbed browser, some properties and methods still 
/// apply to the overall window that contains the tab, such as resizeTo() and innerHeight. 
/// Generally, anything that can't reasonably pertain to a tab pertains to the window instead.
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window
#[wasm_bindgen]
pub fn get_window() -> Result<web_sys::Window, JsValue> {
    match web_sys::window() {
        Some(window) => Ok(window),
        None => Err(JsValue::from_str("No window available..")),
    }
}