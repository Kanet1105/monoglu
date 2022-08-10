use super::Component;

pub struct NavigationBar;

impl NavigationBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl egui::Widget for NavigationBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        ui.horizontal(|ui| {
            ui.button("home");
            ui.button("developer");
        });

        response
    }
}