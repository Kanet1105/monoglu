mod components;
use components::{
    SharedState,
};

mod pages;
use pages::Home;

use yew::prelude::*;
use yew_router::prelude::*;

/// AppRoute defines enum for the router
#[derive(Clone, PartialEq, Routable)]
enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// switch() actually does the work.
fn switch(app_route: &AppRoute) -> Html {
    match app_route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::NotFound => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    use std::rc::Rc;
    let app_state = Rc::new(use_state(|| String::from("Sign In")));

    html! {
        <ContextProvider<SharedState> context={Rc::clone(&app_state)}>
            <BrowserRouter>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<SharedState>>
    }
}