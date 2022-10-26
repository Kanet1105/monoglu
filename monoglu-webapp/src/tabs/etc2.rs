pub struct Etc2;

impl Etc2 {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Etc2 {
    fn name(&self) -> &'static str {
        "Etc.2"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Etc.2");
        });
    }
}