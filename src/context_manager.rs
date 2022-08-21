use crate::prelude::*;

/// ContextManager which serves as a shared reference to the
/// 'T' provided by ContextProvider<T> component where T == State.
/// ContextManager must be called inside the function to which 
/// Context<Self> is provided as an argument of the function.
/// 
/// If you want to implement a pub/sub pattern, refer to the below example.
/// 
/// // lib.rs
/// ```
/// use yew::prelude::*;
/// 
/// pub enum Event {
///     Increment,
///     Decrement,
/// }
/// 
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let manager = ContextManager::new();
/// 
///     html! {
///         <ContextProvider<ContextManager> context={manager.clone()}>
///             <Publisher />
///             <Subscriber />
///         </ContextProvider<ContextManager>>
///     }
/// }
/// 
/// // publisher.rs
/// use crate::Event;
/// use yew::prelude::*;
/// 
/// #[derive(Clone, PartialEq)]
/// pub struct Publisher;
/// 
/// impl Component for Publisher {
///     type Message = Event;
///     type Properties = ();
/// 
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self
///     }
/// 
///     fn view(&self, ctx: &Context<Self>) -> Html {
///         // get the subscriber scope inside view function and add
///         // callbacks to the returned scope. Callbacks are directly
///         // attached to the subscriber scope.
///         let subscriber = ContextManager::get(ctx)
///             .unwrap()
///             .find_scope::<Subscriber>("Subscriber")
///             .unwrap();
/// 
///         let increment = subscriber.callback(|_| Event::Increment);
///         let decrement = subscriber.callback(|_| Event::Decrement);
/// 
///         html! {
///             <div>
///                 <button onclick={increment}>{ "INCREMENT" }</button>
///                 <button onclick={decrement}>{ "DECREMENT" }</button>
///             </div>
///         }
///     }
/// } 
/// 
/// // subscriber.rs
/// use crate::Event;
/// use yew::prelude::*;
/// 
/// #[derive(Clone, PartialEq)]
/// pub struct Subscriber {
///     counter: i32,
/// }
/// 
/// impl Component for Subscriber {
///     type Message = Event;
///     type Properties = ();
/// 
///     fn create(ctx: &Context<Self>) -> Self {
///         // get context manager to which the current Context<Self> 
///         // belong and call share_scope to add the current scope to
///         // to the global state. 
///         ContextManager::get(ctx)
///             .unwrap()
///             .share_scope("Subscriber", ctx).unwrap();
/// 
///         Self { counter: 0, }
///     }
/// 
///     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
///         match msg {
///             Event::Increment => self.counter += 1,
///             Event::Decrement => self.counter -= 1,
///         }
///         true
///     }
/// 
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         html! {
///             <h1>{ self.counter.to_owned() }</h1>
///         }
///     }
/// }
/// 
/// ```
pub struct ContextManager(Rc<RefCell<State>>);

impl ContextManager {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(State::new())))
    }

    /// Get a clone of ContextManager under a given context.
    pub fn get<T>(ctx: &Context<T>) -> Result<Self, String> 
    where
        T: Component,
    {
        match ctx.link().context::<Self>(Callback::noop()) {
            Some(context) => {
                let (manager, _) = context;
                Ok(manager.clone())
            },
            None => Err(format!("No context found..")),
        }
    }

    /// Share the current context which could be accessed by the given id.
    pub fn share_scope<T>(&self, id: &str, ctx: &Context<T>) -> Result<(), String> 
    where
        T: Component,
    {
        let mut state = self.borrow_mut();
        match state.scope_map.contains_key(id) {
            true => Err(format!("Scope (id == {}) already exists..", id)),
            false => {
                let scope = AnyScope::from(ctx.link().clone());
                state.scope_map.insert(id.to_string(), scope);
                Ok(())
            },
        }
    }

    // Find the Scope<T> of struct T by the given id.
    pub fn find_scope<T>(&self, id: &str) -> Result<Scope<T>, String> 
    where
        T: Component,
    {
        let state = self.borrow();
        match state.scope_map.get(id) {
            // downscope does the casting operation from Anyscope to Scope<T> because
            // Anyscope does not implement callback().
            Some(anyscope) => {
                let scope = anyscope.clone().downcast::<T>();
                Ok(scope.clone())
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