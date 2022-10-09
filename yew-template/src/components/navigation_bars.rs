use crate::prelude::*;
use yew::prelude::*;

pub struct GlobalNavBar;

impl Component for GlobalNavBar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container-fluid bg-dark p-1 mb-2">
                <nav class="container-lg navbar navbar-expand-lg navbar-dark">
                    <a class="navbar-brand" href="/">{ "MONOGLU" }</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#GlobalNavBar" aria-controls="GlobalNavBar" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="GlobalNavBar">
                        <ul class="navbar-nav">
                            <a class="nav-link" aria-current="page" href="#">{ "Getting Started" }</a>
                            <a class="nav-link" href="#">{ "Developer's Guide" }</a>
                            <a class="nav-link" href="#">{ "About" }</a>
                        </ul>
                    </div>
                </nav>
            </div>
        }
    }
}
