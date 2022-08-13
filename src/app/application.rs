#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Application {}

impl Application {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = creation_context.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}