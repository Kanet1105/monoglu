pub struct Chat;

impl Chat {
    pub fn new() -> Self {
        Self
    }
}

impl super::Application for Chat {
    fn view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.label("Chatter.");
            });
    }
}
