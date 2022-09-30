use crate::prelude::*;
use yew::prelude::*;

pub struct AuthPopUp;

impl Component for AuthPopUp {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="modal fade" id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="staticBackdropLabel">{ "Sign in to use our service." }</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                    </div>
                    <div class="modal-body">
                        <a class="btn" href="#" role="button">{ "Sign in with Google" }</a>
                        <a class="btn" href="#" role="button">{ "Sign in with Github" }</a>
                    </div>
                </div>
            </div>
        }
    }
}
