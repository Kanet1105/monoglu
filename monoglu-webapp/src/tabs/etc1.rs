pub struct Etc1;

impl Etc1 {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Etc1 {
    fn name(&self) -> &'static str {
        "Etc.1"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Etc.1");
        });
    }
}