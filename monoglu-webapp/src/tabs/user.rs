pub struct User;

impl User {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for User {
    fn name(&self) -> &'static str {
        "User"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame_1 = egui::Frame::none()
                .outer_margin(egui::style::Margin {
                    left: 0.0,
                    right: 0.0,
                    top: 0.0,
                    bottom: 0.0,
                })
                .inner_margin(egui::style::Margin::same(100.0))
                .fill(egui::Color32::BLACK)
                .show(ui, |ui| {
                    ui.label("frame_1");
                });

            ui.horizontal(|ui| {
                let frame_2 = egui::Frame::none()
                    .outer_margin(egui::style::Margin {
                        left: 0.0,
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    })
                    .inner_margin(egui::style::Margin::same(100.0))
                    .fill(egui::Color32::GREEN)
                    .show(ui, |ui| {
                        ui.label("frame_2");
                    });

                let frame_3 = egui::Frame::none()
                    .outer_margin(egui::style::Margin {
                        left: 0.0,
                        right: 0.0,
                        top: 0.0,
                        bottom: 0.0,
                    })
                    .inner_margin(egui::style::Margin::same(100.0))
                    .fill(egui::Color32::GRAY)
                    .show(ui, |ui| {
                        ui.label("frame_3");
                    });
            });
        });
    }
}
