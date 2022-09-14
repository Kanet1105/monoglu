use crate::Exception;
use config;
use sled;
use std::{
    collections::HashMap,
    ops::Deref,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct SharedState(Arc<InnerState>);

impl SharedState {
    pub fn new() -> Result<Self, Exception> {
        let inner_state = InnerState::new()?;
        Ok(Self(Arc::new(inner_state)))
    }
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for SharedState {
    type Target = Arc<InnerState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct InnerState {
    pub storage: sled::Tree,
}

impl InnerState {
    pub fn new() -> Result<Self, Exception> {
        Ok(Self {
            storage: new_storage()?,
        })
    }
}

fn load_config() -> Result<, Exception> {
    
}

// fn new_storage() -> Result<sled::Tree, Exception> {
    
// }
