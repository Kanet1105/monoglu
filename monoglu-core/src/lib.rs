pub mod fetch;
pub mod log;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    let url = "https://web.dev/introduction-to-fetch/";
    let result = fetch::request(url.into()).await?;
    log::log_raw(&result).unwrap();
    Ok(())
}
