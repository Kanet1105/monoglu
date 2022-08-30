mod web;

pub mod prelude {
    pub use crate::init_once;
    pub use crate::web_api;
}

use std::sync::{Arc, Once};
use crate::web::WebAPI;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_logger;
use log::Level;


/// # RESOURCE_HANDLE
/// 
/// This *mut T is actually safe to cast to Arc<T> because they are 
/// read-only pointer to the same web-sys objects that are all Sync 
/// and, therefore, safely shareable among different threads.
/// The *const WebAPI raw pointer is cast into Arc<WebAPI> object via
/// ['web_api()'] function in the current crate.
/// 
/// ```
/// #[wasm_bindgen(start)]
/// pub async fn start_app() -> Result<(), JsValue> {
///     init_once().unwrap();
///     let a = web_api().unwrap();
///     let b = web_api().unwrap();
/// 
///     // Assert if two Arc<WebAPI> point to the same underlying data object.
///     assert!(Arc::ptr_eq(&a, &b));
/// 
///     // Print a log to see the actual address.
///     log::info!("{:?}, {:?}", Arc::as_ptr(&a), Arc::as_ptr(&b));
/// 
///     Ok(())
/// }
/// ```
pub(crate) static mut RESOURCE_HANDLE: Option<*const WebAPI> = None;
static INIT: Once = Once::new();

/// ['crate::init_once()'] provides one-time global initiation constructed 
/// with ['std::sync::Once::new()']. 'Once' can be in one of the following
/// four states.
/// 
/// - const INCOMPLETE: usize = 0x0;
/// - const POISONED: usize = 0x1;
/// - const RUNNING: usize = 0x2;
/// - const COMPLETE: usize = 0x3;
/// 
/// When called at the beginning of the main() block, this function
/// does not need to be called again and if it is, the program will 
/// panic.
/// 
/// Call init_once() only once inside the main block of the calling
/// thread and use the public API to access the web-sys functionalities.
#[wasm_bindgen]
pub fn init_once() -> Result<(), String> {
    // Initialize the logger.
    wasm_logger::init(wasm_logger::Config::new(Level::Info));

    // Initialize the browser resource handle.
    // Panic when already initialized.
    if INIT.is_completed() {
        let message = "Error trying to initialize the browser resource handle twice.";
        log::error!("{:?}", &message);
        panic!();
    } else {
        unsafe {
            INIT.call_once(|| {
                let web_api = Arc::new(WebAPI::new());
                let raw_ptr = Arc::into_raw(web_api);
                RESOURCE_HANDLE = Some(raw_ptr);
            });
        }
        log::info!("Initialize the browser resource handle.");

        Ok(())
    }
}

/// [monoglu_core::web_api()] is a public interface from which the user
/// can get a thread-safe shared pointer Arc<web_sys::Window>.
/// 
/// It is important to note that [web_sys::window()] also provides
/// a safely sharable Javascript Window object, but it is not really
/// convenient to use in the rust code because most of the API call
/// returns [Result<web_sys::T, JsValue>], [Option<T>] or even
/// [Result<Option<T>, JsValue>]. 
/// 
/// To minimize the cumbersome nested error handling and mild overhead
/// of re-creating a javascript window object handle. ['web_sys::Window']
/// is made static and wrapped in [std::sync::Arc] to be easily shareable 
/// inside the rust code. 
pub fn web_api() -> Result<Arc<WebAPI>, ()> {
    if !INIT.is_completed() {
        wasm_logger::init(wasm_logger::Config::new(Level::Info));
        log::error!("{}", "Uninitialized Web API.");
        panic!();
    } else {
        unsafe {
            match RESOURCE_HANDLE {
                Some(raw_ptr) => {
                    let web_api = Arc::from_raw(raw_ptr);
                    Ok(web_api)
                },
                None => {
                    log::error!("{}", "RESOURCE_HANDLE is not available.");
                    panic!();
                },
            }
        }
    }
}

// Examples

// #[wasm_bindgen(start)]
// pub async fn start_app() -> Result<(), JsValue> {
//     init_once().unwrap();
//     let a = web_api().unwrap();
//     let b = web_api().unwrap();
// 
//     // Assert if two Arc<WebAPI> point to the same underlying data object.
//     assert!(Arc::ptr_eq(&a, &b));
// 
//     // Print a log to see the actual address.
//     log::info!("{:?}, {:?}", Arc::as_ptr(&a), Arc::as_ptr(&b));
// 
//     // access the web api through the public interface.
// 
//     Ok(())
// }

#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    init_once().unwrap();
    let api = web_api().unwrap();
    let a = api.average_performance(test1, 10);
    let b = api.average_performance(test2, 10);
    log::info!("{}", a - b);
    Ok(())
}

#[wasm_bindgen]
pub fn test1() {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage()
        .unwrap()
        .unwrap();
    storage.set_item("key", "123").unwrap();
}

#[wasm_bindgen]
pub fn test2() {
    let api = web_api().unwrap();
    let storage = api.local_storage();
    storage.set_item("key", "123");
}