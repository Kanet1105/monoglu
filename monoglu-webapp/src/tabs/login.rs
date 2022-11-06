use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserInfo {
    pub id: String,
    pub password: String,
}

impl UserInfo {
    pub fn new(id: String, password: String) -> Self {
        Self { id, password }
    }
}

pub struct Login {
    id: String,
    password: String,
    token: Arc<Mutex<String>>,
}

impl Login {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            password: String::new(),
            token: Mutex::new(String::new()).into(),
        }
    }
    
    pub fn login(&mut self) {
        let body = UserInfo {
            id: self.id.to_string(),
            password: self.password.to_string(),
        };
        let request = ehttp::Request::post(
            "/auth".to_string(),
            serde_json::to_vec(&body).unwrap(),
        );
        let token = self.token.clone();
        ehttp::fetch(request, move |response| {
            match response {
                Ok(response) => {
                    *token.lock().unwrap() = response
                        .text()
                        .unwrap()
                        .to_string();
                    // log::info!("{:?}", &self.token);
                },
                Err(e) => log::info!("{:?}", e),
            }
        });
    }

    pub fn flush_user_info(&mut self) {
        self.id.clear();
        self.password.clear();
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
                            ui.add(egui::Label::new("ID : "));
                            ui.add(egui::TextEdit::singleline(&mut self.id)
                                .desired_width(f32::INFINITY)
                            );
                            // ui.text_edit_singleline(&mut self.id);
                            ui.end_row();

                            ui.add(egui::Label::new("Password : "));
                            ui.add(egui::TextEdit::singleline(&mut self.password)
                                .desired_width(f32::INFINITY)
                                .password(true)
                            );
                            // ui.text_edit_singleline(&mut self.password);
                            ui.end_row();
                        });

                    ui.add_space(height * 0.02);
                    ui.separator();
                    ui.add_space(height * 0.02);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Cancel").clicked() {
                            self.flush_user_info();
                        };
                        if ui.button("Ok").clicked() {
                            self.login();
                            self.flush_user_info();
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

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}
