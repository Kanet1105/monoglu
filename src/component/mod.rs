pub mod test;
pub use test::Test;

pub mod test1;
pub use test1::Test1;

pub mod test2;
pub use test2::Test2;

pub mod nav;
pub use nav::Navigator;

#[derive(Clone)]
pub enum Route {
    Test,
    Test1,
    Test2,
}

pub trait Component {
    type Router;

    fn new() -> Self;
    fn view(&self, ctx: &egui::Context, event: crate::prelude::Event, state: crate::prelude::State);
}