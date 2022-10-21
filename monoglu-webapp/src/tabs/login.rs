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
        let width = ui.ctx().available_rect().width();
        let height = ui.ctx().available_rect().height();

        egui::Frame::none()
            .stroke(egui::Stroke {
                width: 2.0,
                color: egui::Color32::GRAY,
            })
            .rounding(egui::Rounding::same(10.0))
            .outer_margin(egui::style::Margin::symmetric(
                width * 0.3,
                height * 0.15,
            ))
            .inner_margin(egui::style::Margin::symmetric(
                width * 0.02,
                height * 0.02,
            ))
            .show(ui, |ui| {
                let input_layout = egui::Layout::top_down(egui::Align::Center);
                ui.allocate_ui_with_layout(ui.available_size(), input_layout, |ui| {
                    ui.heading("Sign In");
                    
                    ui.add_space(height * 0.02);
                    ui.separator();
                    ui.add_space(height * 0.02);
                    
                    egui::Grid::new("login_grid")
                        .num_columns(2)
                        .spacing(egui::vec2(width * 0.02, height * 0.04))
                        .show(ui, |ui| {
                            ui.label("ID : ");
                            ui.text_edit_singleline(&mut self.id);
                            ui.end_row();

                            ui.label("Password : ");
                            ui.text_edit_singleline(&mut self.password);
                            ui.end_row();
                        });

                    ui.add_space(height * 0.02);
                    ui.separator();
                    ui.add_space(height * 0.02);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Cancel").clicked() {
                            self.id.clear();
                            self.password.clear();
                        };
                        if ui.button("Ok").clicked() {
                            
                        };
                    });
                });
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
