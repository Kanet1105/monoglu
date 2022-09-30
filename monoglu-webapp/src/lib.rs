mod components;
mod context_manager;
mod events;
mod pages;

/// The module is used globally within the crate.
pub mod prelude {
    pub use crate::{
        components::*,
        context_manager::ContextManager,
    };
    pub use gloo::{
        storage::{LocalStorage, Storage},
        net::http,
    };
}

use crate::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

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
        Route::Home => html! { <AuthPopUp /> },
        Route::NotFound => html! { <h1>{ "404 : Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Initiate <ContextManager> through which components contexts
    // are stored and shared under the same <ContextProvider>.
    let manager = ContextManager::new();

    html! {
        <ContextProvider<ContextManager> context={manager}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<ContextManager>>
    }
}
