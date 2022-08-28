mod components;
mod context_manager;
mod events;
mod pages;

/// The module is used globally within the crate.
pub mod prelude {
    /// std
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use std::fmt::Debug;
    pub use std::fs;
    pub use std::ops::Deref;
    pub use std::rc::Rc;

    /// monoglu
    pub use crate::context_manager::ContextManager;
    pub use crate::events::*;

    /// external
    pub use monoglu_core::prelude::*;
    pub use yew::prelude::*;
    pub use yew::html::{AnyScope, Scope, TargetCast};
    pub use yew_router::prelude::*;
}

use crate::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/dev")]
    Dev,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <pages::Home /> },
        Route::Dev => html! { <pages::DeveloperHome /> },
        Route::NotFound => html! { <h1>{ "404 : Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Initiate <ContextManager> through which components contexts
    // are stored and shared under the same <ContextProvider>.
    let manager = ContextManager::new();

    html! {
        <ContextProvider<ContextManager> context={manager.clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<ContextManager>>
    }
}