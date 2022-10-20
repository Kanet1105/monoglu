pub struct Login {
    id: String,
    password: String,
}

impl Login {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            password: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Sign In");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("ID : ");
                ui.text_edit_singleline(&mut self.id);
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Password : ");
                ui.text_edit_singleline(&mut self.password);
            });
            ui.separator();
        });
    }
}

impl super::Tab for Login {
    fn name(&self) -> &'static str {
        "Login"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}
