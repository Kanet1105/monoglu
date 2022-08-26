use crate::prelude::*;

pub struct LogIn;

impl Component for LogIn {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        html! {
            
        }
    }
}