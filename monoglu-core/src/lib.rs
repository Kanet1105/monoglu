mod web;

pub mod prelude {

}

use wasm_bindgen::prelude::*;
use wasm_logger;
use log::Level;

/// Static instances.
use std::sync::Once;

static INIT: Once = Once::new();
static mut RESOURCE_HANDLE: Option<ResourceHandle> = None;

/// The static browser resource handle
#[wasm_bindgen]
pub struct ResourceHandle {
    window: web_sys::Window,
    local_storage: web_sys::Storage,
    session_storage: web_sys::Storage,
}

/// # Initialization
/// The function provides one-time global initiation constructed with
/// ['std::sync::Once::new()']. 'Once' can be in one of the following
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
#[wasm_bindgen]
pub fn init_once() -> Result<(), String> {
    // Initialize the logger.
    wasm_logger::init(wasm_logger::Config::new(Level::Info));

    // Initialize the browser resource handle.
    // Panic when already initialized.
    if INIT.is_completed() {
        let message = "Error trying to initialize the browser resource handle twice.";
        log::error!("{:?}", &message);
        Err(message.into())
    } else {
        // SAFETY: Call init_once() only once inside the main block and
        // use the public API to access functionalities.
        unsafe {
            INIT.call_once(|| {
                RESOURCE_HANDLE = Some(ResourceHandle {
                    window: window(),
                    local_storage: local_storage(),
                    session_storage: session_storage(),
                });
            });
        }

        log::info!("Initialize the browser resource handle.");
        Ok(())
    }
}

/// # The Window Interface.
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
pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect_throw("Window object is not available.")
}

/// # Local Storage
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
pub(crate) fn local_storage() -> web_sys::Storage {
    window()
        .local_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Local Storage object is not available.")
}

/// # Session Storage
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
pub(crate) fn session_storage() -> web_sys::Storage {
    window()
        .local_storage()
        .expect_throw("Error unwrapping the local storage.")
        .expect_throw("Session Storage object is not available.")
}

#[wasm_bindgen(start)]
pub async fn start_app() -> Result<(), JsValue> {
    // wasm_logger::init(wasm_logger::Config::new(Level::Info));
    init_once().unwrap();
    Ok(())
}

// #[wasm_bindgen]
// pub fn bench() {
//     let window = get_window().cast();
//     let storage = session_storage().cast();
//     let perf = get_perf(&window);

//     let n: usize = 1000;
//     let mut mark: f64 = 0.0;

//     for _ in 0..100 {
//         let diff = test_1(&perf) - test_2(&perf, &storage);
//         mark += diff;
//     }

//     log::debug!("{}", mark / n as f64)
// }

// #[wasm_bindgen]
// pub fn test_1(perf: &web_sys::Performance) -> f64 {
//     let start = perf.now();
//     for _ in 0..100 {
//         let window = web_sys::window().cast_throw("no window..");
//         let window = window.cast();
//         let storage = window.session_storage()
//             .cast_throw("no storage..")
//             .cast();
//         storage.set_item("user", "12345").cast();
//     }
//     let end = perf.now();
//     end - start
// }

// #[wasm_bindgen]
// pub fn test_2(perf: &web_sys::Performance, storage: &Storage) -> f64 {
//     let start = perf.now();
//     for _ in 0..1 {
//         storage.set_item("user", "12345").unwrap();
//     }
//     let end = perf.now();
//     end - start
// }



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