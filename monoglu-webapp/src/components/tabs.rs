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
            <div class="container-xl d-flex justify-content-center border-bottom mt-4 mb-2">
                <ul class="nav">
                    <li class="nav-item me-2">
                        <a class="nav-link active" aria-current="page" href="#">{"Profile"}</a>
                    </li>
                    <li class="nav-item me-2">
                        <a class="nav-link" href="#">{"Repositories"}</a>
                    </li>
                </ul>
            </div>
        }
    }
}