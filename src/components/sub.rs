use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Subscriber {
    counter: i32,
}

impl Component for Subscriber {
    type Message = Event;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { counter: 0, }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::Increment => self.counter += 1,
            Event::Decrement => self.counter -= 1,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ self.counter.to_owned() }</h1>
        }
    }
}