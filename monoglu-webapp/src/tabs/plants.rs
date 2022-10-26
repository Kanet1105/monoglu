pub struct Plants;

impl Plants {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Plants {
    fn name(&self) -> &'static str {
        "Plants"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Plants");
        });
    }
}