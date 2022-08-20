use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Subscriber {
    state: ContextManager,
    counter: i32,
}

impl Component for Subscriber {
    type Message = Event;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (manager, _) = ctx
            .link()
            .context::<ContextManager>(Callback::noop())
            .expect("No context is found..");
        let myself = AnyScope::from(ctx.link().clone());
        manager.insert("Subscriber", myself).unwrap();

        Self {
            state: manager,
            counter: 0,
        }
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