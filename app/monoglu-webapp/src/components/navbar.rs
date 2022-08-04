use crate::components::SharedState;

use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <header class="navbar navbar-expand-lg navbar-dark bg-dark">
            <nav class="container-lg p-1">
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-main" aria-controls="navbar-main" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                // Icon and brand name
                <a class="navbar-brand" href="/">
                    // <img src= "" width="" height="" class="d-inline-block align-top" alt="">
                    { "MONOGLU" }
                </a>
                // Navbar items
                <div class="collapse navbar-collapse" id="navbar-main">
                    <ul class="navbar-nav me-auto">
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "Getting Started" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "Developer" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{ "About" }</a>
                        </li>
                    </ul>
                    // Profile
                    <form class="navbar-nav">
                        <ProfileLogo />
                    </form>
                </div>
            </nav>
        </header>
    }
}

#[function_component(ProfileLogo)]
fn profile_logo() -> Html {
    let context = use_context::<SharedState>().unwrap();

    html! {
        <button class="btn btn-outline-light" type="button">{ context.as_str() }</button>
    }
}