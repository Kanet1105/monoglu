pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Dashboard");
        });
    }
}