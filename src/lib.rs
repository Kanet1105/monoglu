mod components;
mod context_manager;
mod pages;

/// The module is used globally.
pub mod prelude {
    /// std
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use std::fmt::Debug;
    pub use std::ops::Deref;
    pub use std::ptr;
    pub use std::rc::Rc;

    /// monoglu
    pub use crate::pages::test::Test;

    pub use crate::components::card::CardList;
    pub use crate::components::prod::Producer;
    pub use crate::components::sub::Subscriber;

    pub use crate::context_manager::{
        manager, 
        ContextManager, 
        Event, 
        State
    };

    /// external
    pub use gloo_net::http::Request;
    pub use yew::prelude::*;
    pub use yew::html::{AnyScope, Scope};
    pub use yew_router::prelude::*;
}

use crate::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Test /> },
        Route::NotFound => html! { <h1>{ "404 : Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let manager = ContextManager::new();

    html! {
        <ContextProvider<ContextManager> context={manager.clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<ContextManager>>
    }
}