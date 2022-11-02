pub struct Etc;

impl Etc {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Etc {
    fn name(&self) -> &'static str {
        "Etc"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Etc");
        });
    }
}