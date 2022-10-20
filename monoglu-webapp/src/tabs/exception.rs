pub enum Exception {
    PageNotFound,
    InternalError,
}

impl super::Tab for Exception {
    fn name(&self) -> &'static str {
        "Exception"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self {
                Self::PageNotFound => {
                    ui.label("[404] Page Not Found.");
                },
                Self::InternalError => {
                    ui.label("[500] Internal Error.");
                },
            }
        });
    }
}
