use crate::prelude::*;

pub struct StatusBar;

impl Component for StatusBar {
    type Router = Route;

    fn new() -> Self {
        Self {}
    }

    fn view(&self, ctx: &egui::Context, frame: &mut eframe::Frame, event: crate::prelude::Event, state: crate::prelude::State) {
        let now = ctx.input().time;
        let fps = {
            let state_handle = state.lock().unwrap();
            let df = now - state_handle.last_rendered;
            let fps: f64 = 1.0 / df;

            fps
        };

        egui::TopBottomPanel::bottom("StatusBar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Frames Per Second: {:.2}", fps as i32));
            });
        });

        let state_handle = state.clone();
        event.push(move || {
            let mut state_guard = state_handle.lock().unwrap();
            state_guard.last_rendered = now;
        }).unwrap();
    }
}