mod app;
mod event;
mod component;
mod state;

pub mod prelude {
    pub use std::collections::{HashMap, VecDeque};
    pub use std::sync::{Arc, Mutex};

    // component module
    pub use crate::component::{Component, Route};
    pub use crate::component::{Navigator, Test, Test1, Test2};
    // app.rs
    pub use crate::app::Application;

    // event.rs
    pub use crate::event::{Event, EventHandle};

    // state module
    pub use crate::state::{State, StateHandle};
}