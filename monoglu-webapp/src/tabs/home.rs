
pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {  }
    }
}

impl super::Tab for Home {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        egui::CentralPanel::default().show(ctx, |_ui|{
           
        });
    }
}