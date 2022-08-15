mod app;
mod event;
mod pages;

pub mod prelude {
    pub use std::collections::{HashMap, VecDeque};
    pub use std::sync::{Arc, Mutex};

    pub use crate::app::Application;
    pub use crate::event::{Event, EventHandle, EventType};

    pub trait Component {
        fn view(&self, ctx: &egui::Context, event: Event, state: State);
    }

    pub use crate::pages::Test;

    pub type State = Arc<Mutex<StateHandle>>;

    pub struct StateHandle {
        pub counter: i32,
    }
    
    impl StateHandle {
        pub fn new() -> State {
            Arc::new(Mutex::new(
                Self { counter: 0, }
            ))
        }
    }
}

