use crate::prelude::*;

/// Newtype idiom applied in the State struct. Event.rs module has detailed
/// explanation for the reason why we switched from type aliasing.
pub struct State(Arc<Mutex<StateHandle>>);

impl State {
    pub fn new() -> Self {
        // notice the parentheses instead of braces.
        Self(
            Arc::new(Mutex::new(StateHandle::new()))
        )
    }
}

impl Deref for State {
    type Target = Arc<Mutex<StateHandle>>;
     
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// Shared state handle to the values we work with in the runtime.
/// The struct serves as a static key-value pair whose key is a user defined
/// keyword and a value is one of the primitive types. Currently, we are not
/// planning to support generics.
pub struct StateHandle {
    pub counter: i32,
    pub counter1: i32,
    pub counter2: i32,
    pub last_rendered: f64,
}

impl StateHandle {
    pub fn new() -> Self {
        Self {
            counter: 0,
            counter1: 1,
            counter2: 2,
            last_rendered: 0.0,
        }
    }
}