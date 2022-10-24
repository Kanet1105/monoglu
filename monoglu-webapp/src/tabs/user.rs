use crate::cell::{Grid, Cell};

pub struct User {
    grid: Grid,
}

impl User {
    pub fn new() -> Self {
        Self { grid: Grid::new("user_tab_grid", 3, 3) }
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid.show(ctx);
        });
    }
}
