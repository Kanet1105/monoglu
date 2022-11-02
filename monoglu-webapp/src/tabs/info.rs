pub struct Info;

impl Info {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Info {
    fn name(&self) -> &'static str {
        "info"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Info");
        });
    }
}