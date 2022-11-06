mod dashboard;
mod plants;
mod user;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    ops::Deref, 
};

pub struct DataStates {
    pub d1: Test1,
    pub d2: Test2,
}

impl DataStates {
    pub fn new() -> Self {        
        Self {
            d1: Test1::new(),
            d2: Test2::new(),
        } 
    }

    pub fn dispatch <F, S>(data: Box<S>, func: F)
    where
        F: FnOnce(Box<S>) + 'static,
    {
        func(data)
    }
}

impl Default for DataStates {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Test1 {
    pub x: Arc<Mutex<i32>>,
    pub y: Arc<Mutex<i32>>,
}

impl Test1 {
    pub fn new() -> Self {
        Self {
            x: Arc::new(Mutex::new(0)),
            y: Arc::new(Mutex::new(10)),
        }
    }
}

pub struct Test2 {
    pub a: Arc<Mutex<f32>>,
    pub b: Arc<Mutex<f32>>,
}

impl Test2 {
    pub fn new() -> Self {
        Self {
            a: Arc::new(Mutex::new(100.1)),
            b: Arc::new(Mutex::new(-100.1)),
        }
    }
}

