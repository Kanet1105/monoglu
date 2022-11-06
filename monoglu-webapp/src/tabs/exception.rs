pub enum Exception {
    PageNotFound,
    InternalError,
    RequestTimeOut,
}

impl super::Tab for Exception {
    fn name(&self) -> &'static str {
        "Exception"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self {
                Self::PageNotFound => {
                    ui.heading("[404] Page Not Found.");
                },
                Self::InternalError => {
                    ui.heading("[500] Internal Error.");
                },
                Self::RequestTimeOut => {
                    ui.heading("Request Timeout.");
                }
            }
        });
    }
}
