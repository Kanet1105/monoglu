use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

pub type SharedState = Rc<State>;

/// use_refmut with a State boilerplate.
pub fn init_shared_state() -> SharedState {
    Rc::new(State::default())
}

/// The hook is called inside a function component to use context
/// as a local state.
pub fn use_shared_state() -> UseStateHandle<SharedState> {
    let context = use_context::<SharedState>().unwrap();
    use_state(|| {Rc::clone(&context)})
}

#[derive(Clone, PartialEq)]
pub struct State {
    session: RefCell<bool>,
    user_name: RefCell<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            session: RefCell::new(false),
            user_name: RefCell::new(String::new()),
        }
    }
}

impl State {
    pub fn change_user(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut user_name = self.user_name.try_borrow_mut()?;
        *user_name = name.to_string();
        Ok(())
    }
}