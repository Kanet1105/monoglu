use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn get(url: String) -> Result<JsValue, JsValue> {
    let mut options = web_sys::RequestInit::new();
    options.method("GET");
    options.mode(web_sys::RequestMode::SameOrigin);
    let request = web_sys::Request::new_with_str_and_init(&url, &options)?;

    let window = super::window()?;
    let promise = window.fetch_with_request(&request);
    let response = JsFuture::from(promise).await?;

    if response.is_instance_of::<web_sys::Response>() {
        let value: web_sys::Response = response.dyn_into()?;
        let json_value = JsFuture::from(value.json()?).await?;
        Ok(json_value)
    } else {
        let message = "Response of the fetch request is not an instance of ['web_sys::Response'].";
        Err(JsValue::from_str(message))
    }
}

#[wasm_bindgen]
pub async fn post(url: String) -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct RequestBody;