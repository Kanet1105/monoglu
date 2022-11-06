use crate::gridlayout::*;

pub struct User {
    grid_layout: GridLayout,
    selected_achor: String,
}

impl User {
    pub fn new() -> Self {
        let mut grid_layout = GridLayout::new("user_grid".into(), 2, 3, SizePolicy::responsive(1.0, 1.0));
        grid_layout.set_y_stretch(1, 5).unwrap();
        // grid_layout.set_column_stretch(1, 8).unwrap();
        grid_layout
            .get_grid(0, 0)
            .unwrap()
            .add_contents(Box::new(|ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    if ui.button("home").clicked() {
                        ui.output().open_url("/#home");
                    };
                });
            }));
        grid_layout
            .get_grid(0, 1)
            .unwrap()
            .add_contents(Box::new(|ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    if ui.button("dashboard").clicked() {
                        ui.output().open_url("/#dashboard");
                    };
                });
            }));
        grid_layout
            .get_grid(1, 0)
            .unwrap()
            .add_contents(Box::new(|ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    if ui.button("info").clicked() {
                        ui.output().open_url("/#info");
                    };
                });
            }));
        grid_layout
            .get_grid(1, 1)
            .unwrap()
            .add_contents(Box::new(|ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    if ui.button("etc").clicked() {
                        ui.output().open_url("/#etc");
                    };
                });
            }));
        

        Self { 
            grid_layout,
            selected_achor: "A".to_owned(),
        }
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _value_states: &crate::data::DataStates) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                egui::widgets::global_dark_light_mode_switch(ui);
                let list = [("a", "A"), ("b", "B"), ("c", "C")];
                for (name, anchor) in list {
                    if ui.add(
                        crate::monoglu_features::SelectableLabel::new(self.selected_achor == anchor, name)
                            .custom_size(egui::vec2(50.0, 30.0))
                            .fill(egui::Color32::GOLD)
                            .stroke(1.0, egui::Color32::DARK_RED)
                            .rounding(egui::epaint::Rounding::same(25.0))

                        ).clicked()
                    {
                        self.selected_achor = anchor.to_owned();
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.grid_layout.show(ctx);
        });
    }
}
