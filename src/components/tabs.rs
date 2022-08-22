use crate::prelude::*;

pub struct DevTab;

impl Component for DevTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container-xl">
                <ul class="nav justify-center">
                    <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="#">{"Active"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"Link"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" href="#">{"Link"}</a>
                    </li>
                </ul>
            </div>
        }
    }
}