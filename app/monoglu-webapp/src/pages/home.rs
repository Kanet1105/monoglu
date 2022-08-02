use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <nav class="navbar navbar-expand-xl navbar-dark bg-dark">
            <a class="navbar-brand" href="/home">
                // <img src= "" width="" height="" class="d-inline-block align-top" alt="">
                { "MONOGLU" }
            </a>

            <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>

            <div class="collapse navbar-collapse" id="navbarNav">
                <ul class="navbar-nav">
                    <li class="nav-item">
                        <a></a>
                    </li>
                    <li class="nav-item">
                    </li>
                    <li class="nav-item">
                    </li>
                </ul>
            </div>
        </nav>
    }
}