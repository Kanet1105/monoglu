pub trait Tab {
    fn view(&self, ctx: &egui::Context);
}