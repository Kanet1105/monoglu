pub struct Home;

impl Home {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Home {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Home");
        });
    }
}