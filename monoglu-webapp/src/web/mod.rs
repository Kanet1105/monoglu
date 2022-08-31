mod data;
pub use data::Data;

pub mod http;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub fn window() -> Result<web_sys::Window, JsValue> {
    match web_sys::window() {
        Some(window) => Ok(window),
        None => Err(JsValue::from_str("['Window'] object is not available.")),
    }
}