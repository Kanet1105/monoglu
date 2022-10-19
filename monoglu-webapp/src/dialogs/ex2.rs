pub struct Ex2 {
    visible: bool,
}

impl Ex2 {
    pub fn new() -> Self {
        Self { visible: false }
    }
}

impl super::Dialog for Ex2 {
    fn name(&self) -> &'static str {
        "Ex2"
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, state: bool) {
        self.visible = state
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.label("Ex2");
            });
    }
}
