use crate::prelude::*;

pub struct StatusBar;

impl Component for StatusBar {
    type Router = Route;

    fn new() -> Self {
        Self {}
    }

    fn view(&self, ctx: &egui::Context, frame: &mut eframe::Frame, event: crate::prelude::Event, state: crate::prelude::State) {
        let fps: f32 = 0.0;

        egui::TopBottomPanel::bottom("StatusBar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Frames Per Second: {:.2}", fps));
            });
        });

        let state_handle = state.clone();
        event.push(move || {
            let mut state_guard = state_handle.lock().unwrap();
        });
    }
}