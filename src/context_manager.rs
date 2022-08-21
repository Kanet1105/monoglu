use crate::prelude::*;

/// The context manager which wraps State with the basic
/// interior mutability pattern.
pub struct ContextManager(Rc<RefCell<State>>);

impl ContextManager {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(State::new())))
    }

    pub fn insert<T>(&self, id: &str, ctx: &Context<T>) -> Result<(), String> 
    where
        T: Component,
    {
        let scope = AnyScope::from(ctx.link().clone());
        let mut state = self.borrow_mut();
        match state.scope_map.contains_key(id) {
            true => Err(format!("Scope (id == {}) already exists..", id)),
            false => {
                state.scope_map.insert(id.to_string(), scope);
                Ok(())
            },
        }
    }

    pub fn find<T>(&self, id: &str, component: T) -> Result<Scope<T>, String> 
    where
        T: Component,
    {
        let state = self.borrow();
        match state.scope_map.get(id) {
            // downscope does the casting operation from Anyscope to Scope<T> because
            // Anyscope does not implement callback().
            Some(anyscope) => {
                let scope = anyscope.downcast::<T>();
                Ok(scope)
            },
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

/// Get a clone of ContextManager under a given context.
/// It is used to call the current context manager 'T' provided by ContextProvider<T>.
pub fn manager<T, C>(ctx: &Context<T>) -> Result<C, String>
where
    T: Component,
    C: Clone + Deref + PartialEq + 'static,
{
    match ctx.link().context::<C>(Callback::noop()) {
        Some(context) => {
            let (manager, _) = context;
            Ok(manager.clone())
        },
        None => Err(format!("No context found..")),
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