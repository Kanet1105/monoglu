mod components;

mod pages;
use pages::Home;

mod util;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(app_route: &AppRoute) -> Html {
    match app_route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::NotFound => html! {},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}