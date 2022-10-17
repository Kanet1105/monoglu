mod chat;

pub trait View {
    fn show(ctx: &egui::Context);
}

#[derive(serde::Deserialize, serde::Serialize)]
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
