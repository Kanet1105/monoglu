use crate::prelude::*;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};


/// The global fetch() method starts the process of fetching a resource from the network, 
/// returning a promise which is fulfilled once the response is available.
#[wasm_bindgen]
pub async fn http_get(url: String) -> Result<JsValue, JsValue> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(&url, &options)?;

    let window = get_window()?;
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    if response.is_instance_of::<Response>() {
        let value = response.dyn_into::<Response>()?;
        let value = JsFuture::from(value.json()?).await?;
        Ok(value)
    } else {
        Err(response)
    }
}

#[wasm_bindgen]
pub async fn http_post(url: String, body: String) -> Result<JsValue, JsValue> {
    let mut options = RequestInit::new();
    options.method("POST");
    options.mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(&url, &options)?;

    let window = get_window()?;
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    if response.is_instance_of::<Response>() {
        let value = response.dyn_into::<Response>()?;
        let value = JsFuture::from(value.json()?).await?;
        Ok(value)
    } else {
        Err(response)
    }
}

#[wasm_bindgen]
pub fn fetch_request(url: String) {
    spawn_local(async move {
        let response = http_get(url)
            .await
            .unwrap();
        log::debug!("{:?}", response);
    });
}