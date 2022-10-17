mod chat;
mod login;

pub trait View {
    fn show(ctx: &egui::Context);
}

pub struct AppState {}
