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

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Info");
        });
    }
}