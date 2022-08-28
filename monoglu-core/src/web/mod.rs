mod performance;
mod storage;
mod task;
mod window;

pub(crate) use window::window;
pub use storage::{Storage, StorageType}; 

pub struct WebAPI {
    pub window: web_sys::Window,
    pub local_storage: Storage,
    pub session_storage: Storage,
}

impl WebAPI {
    pub fn new() -> Self {
        Self {
            window: window(),
            local_storage: Storage::new(StorageType::Local),
            session_storage: Storage::new(StorageType::Session),
        }
    }
}