use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub content: String,
}

#[wasm_bindgen]
pub async fn request(url: String) -> Result<Response, JsValue> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &options)?;

    let window = web_sys::window().expect("No window available..");
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    if response.is_instance_of::<Response>() {
        let value = response.dyn_into::<Response>().unwrap();
        Ok(value)
    } else {
        Err(JsValue::from_str("fetch failed.."))
    }
}
