use crate::prelude::*;

#[function_component(Test)]
pub fn test() -> Html {
    html! {
        <>
        <Producer />
        <Subscriber />
        </>
    }
}