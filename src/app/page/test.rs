pub struct Test;

impl Test {
    pub fn view(ctx: &egui::Context, state: &crate::app::AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(state.get_counter());
            ui.horizontal(|ui| {
                let increment = ui.button("increment");
                if increment.clicked() {
                    state.increment();
                };
                let decrement = ui.button("decrement");
                if decrement.clicked() {
                    state.decrement();
                };
            });
        });
    }
}