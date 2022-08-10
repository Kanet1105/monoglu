/// All custom components (made from a combination of base widgets / custom widgets or both)
/// should go here as a Struct that implements Component trait.

// All components must implement this trait.
trait Component {
    fn new(ui: &mut egui::Ui) -> egui::Response;
}

mod navigation_bar;
pub use navigation_bar::NavigationBar;