pub struct Login {
    email: String,
    password: String,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

impl super::Application for Login {
    fn view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("SIGN IN");
                        ui.separator();
                        ui.text_edit_singleline(&mut self.email);
                        ui.text_edit_singleline(&mut self.password);
                    });
                });
            });
    }
}
