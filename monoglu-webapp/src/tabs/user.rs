use crate::cell::Cell;

pub struct User;

impl User {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut cell = Cell::new("cell_grid", 2, 2);
            let area_0_0 = cell
                .get_area(0, 0);
            // cell.get_frame(0, 0).fill = egui::Color32::BLUE;
            // cell.get_frame(0, 1).fill = egui::Color32::RED;
            // cell.get_frame(1, 0).fill = egui::Color32::GREEN;
            // cell.get_frame(1, 1).fill = egui::Color32::BLACK;
        });
    }
}
