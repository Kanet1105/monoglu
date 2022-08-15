mod app;
mod event;
mod component;
mod route;
mod state;

pub mod prelude {
    pub use std::collections::{HashMap, VecDeque};
    pub use std::sync::{Arc, Mutex};

    // app.rs
    pub use crate::app::Application;

    // component module
    pub use crate::component::Component;
    pub use crate::component::page;

    // event.rs
    pub use crate::event::{Event, EventHandle};

    // route.rs
    pub use crate::route::{switch, Route};

    // state module
    pub use crate::state::{State, StateHandle};
}