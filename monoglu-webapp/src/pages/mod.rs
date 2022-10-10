mod login;

pub trait View {
    fn show(ctx: &egui::Context);
}