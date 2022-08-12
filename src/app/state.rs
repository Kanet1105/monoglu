use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    counter: Arc<Mutex<i32>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_counter(&self) -> String {
        let counter = self.counter.lock().unwrap();
        counter.to_string()
    }

    pub fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1
    }

    pub fn decrement(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter -= 1
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}