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
                <input class="form-control" type="search" placeholder="Search" />
            </div>
        </div>
    }
}

#[function_component(SearchOption)]
fn search_option() -> Html {
    html! {}
}