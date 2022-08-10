/// All custom components (made from a combination of base widgets / custom widgets or both)
/// should belong in here as structs that implement Component trait.
/// 
/// When a new component is added via new(), it takes a mutable reference to egui::Ui and
/// a global state handle. All components must return egui::Response type.
/// 
/// Components are just an abstraction of a widget creation in 4 steps.
/// 1. Decide a size for the widget.
/// 2. Allocate space for the widget.
/// 3. Handle interactions with the widget.
/// 4. Paint the widget.
/// 
/// Example: https://github.com/emilk/egui/blob/master/egui_demo_lib/src/demo/toggle_switch.rs
trait Component {
    // fn new(ui: &mut egui::Ui, state: StateHandle) -> egui::Response;
    fn new(ui: &mut egui::Ui) -> egui::Response;
}

mod navigation_bar;
pub use navigation_bar::NavigationBar;