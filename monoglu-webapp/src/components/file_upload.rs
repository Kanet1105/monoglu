use crate::prelude::*;

pub struct UploadFile;

impl Component for UploadFile {
    type Message = FileEvent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileEvent::Uploaded => {
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}

pub struct UploadDirectory;

impl Component for UploadDirectory {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}