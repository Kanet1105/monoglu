pub struct CompanyInfo;

impl CompanyInfo {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for CompanyInfo {
    fn name(&self) -> &'static str {
        "Company info"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Company Info");
        });
    }
}