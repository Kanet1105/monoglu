use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Producer;

impl Component for Producer {
    type Message = Event;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let increment = subscriber.callback(|_| Event::Increment);
        // let decrement = subscriber.callback(|_| Event::Decrement);

        html! {
            <>
            <div>
                // <button onclick={increment}>{ "INCREMENT" }</button>
                // <button onclick={decrement}>{ "DECREMENT" }</button>
            </div>

            <div>
                <button>{ "GET" }</button>
                <button>{ "POST" }</button>
            </div>
            </>
        }
    }
}