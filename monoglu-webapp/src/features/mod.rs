mod router;
pub use router::Router;

use std::{
    cell::RefCell, ops::Deref, rc::Rc,
};

pub struct State(Rc<Shared>);

impl State {
    pub fn new() -> Self {
        Self(Rc::new(Shared))
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Deref for State {
    type Target = Rc<Shared>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Shared {
    router: Router,
}
