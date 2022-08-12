use crate::app::page::Test;
use crate::app::state::AppState;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Application {
    state: AppState,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            state: AppState::new(),
        }
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main_layout").show(ctx, |ui| {
            Test::view(ctx, &self.state);
        });
    }
}
