use crate::components::NavBar;
use crate::components::SearchBar;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <NavBar />
        <SearchBar />
        </>
    }
}