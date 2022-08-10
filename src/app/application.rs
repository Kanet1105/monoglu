use crate::app::page::Test;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Application {
}

impl Default for Application {
    fn default() -> Self {
        Self {}
    }
}

impl Application {
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = creation_context.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main_layout").show(ctx, |ui| {
            Test::view(ctx);
        });
    }
}