use crate::prelude::*;

pub struct Test;

impl Component for Test {
    type Router = Route;

    fn new() -> Self {
        Self {}
    }

    fn view(&self, ctx: &egui::Context, frame: &mut eframe::Frame, event:&Event, state: &State) {
        widget::Navigator::new().view(ctx, frame, event, state);
        widget::StatusBar::new().view(ctx, frame, event, state);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Test page");
            let counter = {
                let state_handle = state.lock().unwrap();
                (state_handle.counter, state_handle.counter1, state_handle.counter2)
            };
            
            ui.label(counter.0.to_string());
            ui.label(counter.1.to_string());
            ui.label(counter.2.to_string());

            ui.horizontal(|ui| {
                let increment = ui.button("increment");
                if increment.clicked() {
                    let state_handle = state.clone();

                    event.push(move || {
                        let mut state_guard = state_handle.lock().unwrap();
                        state_guard.counter += 1;
                    }).unwrap();
                };

                let decrement = ui.button("decrement");
                if decrement.clicked() {
                    let state_handle = state.clone();
                    
                    event.push(move || {
                        let mut state_guard = state_handle.lock().unwrap();
                        state_guard.counter -= 1;
                    }).unwrap();
                }
            });
        });
    }
}