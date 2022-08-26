mod fetch;
mod storage;

pub mod prelude {
    pub use crate::get_window;
    pub use crate::fetch::{http_get, http_post};
    pub use crate::storage::{
        local_storage, 
        session_storage
    };
}

use crate::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    http_get("127.0.0.1:8080".into()).await?;
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

// Proof : The localstorage is not thread safe. Run below code in multiple tabs to see why.
// #[wasm_bindgen(start)]
// pub async fn start_app() -> Result<(), JsValue> {
//     wasm_logger::init(wasm_logger::Config::default());
//     let local = Storage::new(StorageType::Local)?;
//     local.set_item("counter", "0").unwrap();
//     // looping().await?;
//     let value = local.get_item("counter")?;
//     log::debug!("{}", value);
//     Ok(())
// }    

// #[wasm_bindgen]
// pub async fn looping() -> Result<(), JsValue> {
//     let storage = Storage::new(StorageType::Local)?;
//     for index in 0..1000 {
//         let value = storage.get_item("counter")?;
//         let mut counter = value.parse::<i32>().unwrap();
//         counter += 1;
//         storage.set_item("counter", &counter.to_string())?;
//     }
//     Ok(())
// }