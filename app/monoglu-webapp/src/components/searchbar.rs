use crate::components::SharedState;

use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, EventTarget};
use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    html! {
        <div class="container-lg my-3">
            <div class="input-group w-50">
                <button class="btn btn-outline-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">{ "Dropdown" }</button>
                <ul class="dropdown-menu">
                    <li><a class="dropdown-item" href="#">{ "1" }</a></li>
                    <li><a class="dropdown-item" href="#">{ "2" }</a></li>
                    <li><hr class="dropdown-divider" /></li>
                    <li><a class="dropdown-item" href="#">{ "3" }</a></li>
                </ul>
                <SearchForm />
            </div>
        </div>
    }
}
#[function_component(SearchForm)]
fn search_form() -> Html {
    let context = use_context::<SharedState>().unwrap();

    let oninput = {
        let context = std::rc::Rc::clone(&context);
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap();
            context.set(input.value());
        })
    };

    html! {
        <input {oninput} class="form-control" type="search" placeholder={ "Search" } value={ context.to_string() } />
    }
}