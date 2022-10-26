pub struct TechInfo;

impl TechInfo {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for TechInfo {
    fn name(&self) -> &'static str {
        "Tech Info"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Tech Info");
        });
    }
}