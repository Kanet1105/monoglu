use crate::prelude::*;

pub struct DeveloperHome;

impl Component for DeveloperHome {
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