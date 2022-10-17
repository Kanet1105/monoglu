mod chat;
mod login;

pub trait View {
    fn show(ctx: &egui::Context);
}

pub struct AppState {}

pub struct WebApp;

impl WebApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for WebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}
