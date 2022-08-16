/// Define modules here.
/// Even "page" is a subset of "component".
/// Every module, whether being a page or a component, is defined at the top of the mod.rs file and 
/// if the module is a page component, it should also be defined inside the "pages" module.
pub mod nav;
pub mod status_bar;
pub mod test;
pub mod test1;
pub mod test2;

/// Define pages for the router. The "switch()" function in the router.rs refers to this module for
/// redirecting a user to a page.
pub mod page {
    pub use super::test::Test;
    pub use super::test1::Test1;
    pub use super::test2::Test2;
}

pub mod widget {
    pub use super::nav::Navigator;
    pub use super::status_bar::StatusBar;
}

/// "Component" is the building block of user interfaces used in the program.
/// Component trait guarantees that the programmer does not make a mistake when building a custom
/// user interface made up of widgets.
pub trait Component {
    type Router;

    fn new() -> Self;
    fn view(
        &self, ctx: &egui::Context, 
        frame: &mut eframe::Frame, 
        event: crate::prelude::Event, 
        state: crate::prelude::State
    );
}