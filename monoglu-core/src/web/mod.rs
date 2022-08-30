mod storage;
pub use storage::{WebStorage, WebStorageType};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
/// # Web API
/// 
/// When writing code for the Web, there are a large number of 
/// Web APIs available and most of them are accessed with ['web_sys::Window'].
/// 
/// ['WebAPI'] module is a high level Rust wrapper to the 'Window'
/// javascript object, which provides the access to the thread-safe, 
/// read-only and static Javascript window reference while avoiding
/// the constant ['web_sys::window()'] function call.
pub struct WebAPI {
    window: web_sys::Window,
    pub local_storage: WebStorage,
    pub session_storage: WebStorage,
}

impl WebAPI {
    /// # The Window Interface.
    /// 
    /// The Window interface represents a window containing a DOM document; 
    /// the document property points to the DOM document loaded in that window.
    ///  
    /// A window for a given document can be obtained using the document.
    /// defaultView property.
    ///  
    /// A global variable, window, representing the window in which the 
    /// script is running, is exposed to JavaScript code.
    /// 
    /// The Window interface is home to a variety of functions, namespaces, 
    /// objects, and constructors which are not necessarily directly 
    /// associated with the concept of a user interface window. However, 
    /// the Window interface is a suitable place to include these items that 
    /// need to be globally available. Many of these are documented in the 
    /// JavaScript Reference and the DOM Reference.
    /// 
    /// In a tabbed browser, each tab is represented by its own Window object; 
    /// the global window seen by JavaScript code running within a given tab 
    /// always represents the tab in which the code is running. That said, 
    /// even in a tabbed browser, some properties and methods still apply to the 
    /// overall window that contains the tab, such as resizeTo() and innerHeight. 
    /// Generally, anything that can't reasonably pertain to a tab pertains to 
    /// the window instead.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window
    pub fn new() -> Self {
        let window = match web_sys::window() {
            Some(value) => value,
            None => {
                log::error!("Window object is unavailable");
                panic!();
            },
        };

        let local_storage = WebStorage::new(&window, WebStorageType::Local);
        let session_storage = WebStorage::new(&window, WebStorageType::Local); 

        Self {
            window,
            local_storage,
            session_storage,
        }
    }

    /// # Using the Fetch API 
    /// 
    /// The Fetch API provides a JavaScript interface for accessing and 
    /// manipulating parts of the HTTP pipeline, such as requests and responses. 
    /// It also provides a global fetch() method that provides an easy, 
    /// logical way to fetch resources asynchronously across the network.
    /// 
    /// This kind of functionality was previously achieved using XMLHttpRequest. 
    /// Fetch provides a better alternative that can be easily used by other 
    /// technologies such as Service Workers. Fetch also provides a single 
    /// logical place to define other HTTP-related concepts such as CORS and 
    /// extensions to HTTP.
    pub fn fetch(&self) {}

    pub fn spawn_task(&self) {}

    /// # Local Storage
    /// 
    /// local storage getter.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
    pub fn local_storage(&self) -> &WebStorage {
        &self.local_storage
    }

    /// # Session Storage
    /// 
    /// Static session storage getter.
    /// 
    /// https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage
    pub fn session_storage(&self) -> &WebStorage {
        &self.session_storage
    }
    
    pub fn average_performance<F>(&self, target: F, repetition: usize) -> f64
    where 
        F: Fn() + 'static,
    {
        let perf_counter = match self.window.performance() {
            Some(counter) => counter,
            None => {
                let message = "Performance is not available.";
                log::error!("{:?}", message);
                panic!("{:?}", message);
            },
        };
        
        let mut history: f64 = 0.0;
            for _ in 0..repetition {
                let start = perf_counter.now();
                target();
                let elapsed = perf_counter.now() - start;
                history += elapsed;
            }

        history / repetition as f64
    }

    

    // pub fn fetch(&self, url: &str, method: Method) -> Result<(), String> {
    //     // build a header with given method and mode arguments.
    //     let mut options = web_sys::RequestInit::new();

    //     match method {
    //         Method::Get => options.method("GET"),
    //         Method::Post => options.method("POST"),
    //     };

    //     match mode {
    //         Mode::Cors => options.mode(web_sys::RequestMode::Cors),
    //         Mode::NoCors => options.mode(web_sys::RequestMode::NoCors),
    //         Mode::SameOrigin => options.mode(web_sys::RequestMode::SameOrigin),
    //     };

    //     // build request with the given optional parameters.
    //     let new_request = match web_sys::Request::new_with_str_and_init(url, &options) {
    //         Ok(request) => request,
    //         Err(error) => {
    //             log::error!("{:?}", error);
    //             panic!("Error unwrapping ['web_sys::Request::new_with_str_and_init()']");
    //         }
    //     };

    //     let promise = JsFuture::from(self.window.fetch_with_request(options));

    //     match promise.await {
    //         Ok(response) => {
    //             let response = response.dyn_into::<web_sys::Response>().unwrap();
    //             let json = JsFuture::from(response.json().unwrap()).await.unwrap();
    //             log::info!("{:?}", json);
    //         },
    //         Err(error) => {
    //             log::error!("{:?}", error);
    //             panic!();
    //         },
    //     }

    //     Ok(())
    // }

    // pub async fn task_spawn(&self, task: JsFuture) {
    //     spawn_local(async move {
    //         let response = task.await.unwrap();
    //         if response.is_instance_of::<web_sys::Response>() {
    //             // let result = response.dyn_into::<web_sys::Response>().unwrap();
    //             let parsed = web_sys::Response::from(response);
                
    //         } else {
    //             log::error!("Unable to fetch..");
    //         }
    //     });
    // }
}

pub enum Method {
    Get,
    Post,
}

pub enum Mode {
    Cors,
    NoCors,
    SameOrigin,
}

pub enum Format {
    Default,
    Raw,
}