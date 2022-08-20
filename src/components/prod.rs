use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Producer {
    state: ContextManager,
}

impl Component for Producer {
    type Message = Event;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (manager, _) = ctx
            .link()
            .context::<ContextManager>(Callback::noop())
            .expect("No context is found..");
        let myself = AnyScope::from(ctx.link().clone());
        manager.insert("Producer", myself).unwrap();

        Self {
            state: manager,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let scope = self.state.get("Subscriber")
            .unwrap()
            .downcast::<Subscriber>();
        let increment = scope.callback(|_| Event::Increment);
        let decrement = scope.callback(|_| Event::Decrement);

        html! {
            <div>
                <button onclick={increment}>{ "INCREMENT" }</button>
                <button onclick={decrement}>{ "DECREMENT" }</button>
            </div>
        }
    }
}