pub trait Dialog {
    fn show(&self, ctx: &egui::Context);
}