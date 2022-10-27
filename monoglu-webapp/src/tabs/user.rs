use crate::cell::Grid;

pub struct User {
    grid: Grid,
    selected_achor: String,
}

impl User {
    pub fn new() -> Self {
        Self { 
            grid: Grid::new("user_tab_grid", 3, 3),
            selected_achor: "A".to_owned(),
        }
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            self.grid.show(ctx);


            self.grid.get_cell(0, 0)
                .unwrap()
                .add_contents(Box::new(|ui| {
                    ui.label("This is the top - left cell. 👾 👽 😫🐙 📊 👤 ⛃ 🌾⚘ 🖧 💡 🏢 🔧 🔍 ➕", 	);
                }));

            self.grid.get_cell(1, 1)
                .unwrap()
                .add_contents(Box::new(|ui| {
                    ui.label(egui::RichText::new("sdff👾 👽 😫🐙 📊 👤 ⛃ 🌾⚘ 🖧 💡 🏢 🔧 🔍 ➕")
                        .text_style(egui::TextStyle::Name("emoji_big".into()))
                    );
                }));
        });
    }
}


