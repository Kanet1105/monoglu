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

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("etc.");
        });
    }
}