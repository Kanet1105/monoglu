/// Pages are a group of components that together make a page functionally coherent.
/// Every page must implements Page trait.
pub trait Page {
    // pub fn new(ctx: &egui::Context, state: StateHandle);
    fn view(ctx: &egui::Context);
}

pub mod test;
pub use test::Test;