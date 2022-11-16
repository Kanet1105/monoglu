mod configuration;

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

pub struct AppData(Arc<Shared>);

impl AppData {
    pub fn new() -> Self {
        Self(Arc::new(Shared::new()))
    }
}

impl Clone for AppData {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for AppData {
    type Target = Arc<Shared>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Shared {
    pub counter: u32,
}

impl Shared {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}
