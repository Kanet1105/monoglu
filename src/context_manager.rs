use crate::prelude::*;

/// The context manager which wraps State with the basic
/// interior mutability pattern.
pub struct ContextManager(Rc<RefCell<State>>);

impl ContextManager {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(State::new())))
    }

    pub fn insert(&self, id: &str, scope: AnyScope) -> Result<(), String> {
        let mut state = self.borrow_mut();

        match state.scope_map.contains_key(id) {
            true => Err(format!("Scope (id == {}) already exists..", id)),
            false => {
                state.scope_map.insert(id.to_string(), scope);
                Ok(())
            },
        }
    }

    pub fn get(&self, id: &str) -> Result<AnyScope, String> {
        let state = self.borrow();
        match state.scope_map.get(id) {
            Some(scope) => return Ok(scope.clone()),
            None => return Err(format!("Scope (id == {}) does not exist..", id)),
        }
    }
}

impl Clone for ContextManager {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Deref for ContextManager {
    type Target = Rc<RefCell<State>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for ContextManager {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Application events are listed under Event type.
pub enum Event {
    Increment,
    Decrement,
}

#[derive(Clone, Debug)]
pub struct State {
    pub id: &'static str,
    pub scope_map: HashMap<String, AnyScope>,
}

impl State {
    pub fn new() -> Self {
        Self {
            id: "State",
            scope_map: HashMap::default(),
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}