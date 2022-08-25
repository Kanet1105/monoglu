use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;

/// Print the log from a rust string slice.
pub fn log(text: &str) {
    let text = JsValue::from_str(text);
    console::log_1(&text);
}

/// Print the log directly from JsValue. Otherwise, return Err(String).
pub fn log_raw(text: &JsValue) -> Result<(), String> {
    if text.is_instance_of::<JsValue>() {
        console::log_1(text);
        Ok(())
    } else {
        Err(format!("The input is not a valid JsValue.."))
    }
}