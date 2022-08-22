use crate::prelude::*;

pub struct Developer;

impl Component for Developer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <crate::components::GlobalNavBar />
                <crate::components::DevTab />
            </>
        }
    }
}