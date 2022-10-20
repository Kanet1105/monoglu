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
            ui.label(self.name());
        });
    }
}
