use wasm_bindgen::prelude::*;

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
    match web_sys::window() {
        Some(handle) => handle,
        None => {
            log::error!("Window object unavailable");
            panic!();
        },
    }
}