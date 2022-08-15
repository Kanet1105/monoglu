use crate::prelude::*;

pub struct Test1;

impl Component for Test1 {
    type Router = Route;

    fn new() -> Self {
        Self {}
    }

    fn view(&self, ctx: &egui::Context, event: Event, state: State) {
        page::Navigator::new().view(ctx, event.clone(), state.clone());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Test1 page");
            let counter = {
                let state_handle = state.lock().unwrap();
                (state_handle.counter, state_handle.counter1, state_handle.counter2)
            };
            
            ui.label(counter.0.to_string());
            ui.label(counter.1.to_string());
            ui.label(counter.2.to_string());

            ui.horizontal(|ui| {
                let event_handle = event.clone();

                let increment = ui.button("increment");
                if increment.clicked() {
                    let state_handle = state.clone();

                    event_handle.push(move|| {
                        let mut state_guard = state_handle.lock().unwrap();
                        state_guard.counter += 1;
                        state_guard.counter1 += 1;
                    }).unwrap();
                };

                let decrement = ui.button("decrement");
                if decrement.clicked() {
                    let state_handle = state.clone();

                    event_handle.push(move|| {
                        let mut state_guard = state_handle.lock().unwrap();
                        state_guard.counter -= 1;
                        state_guard.counter1 -= 1;
                    }).unwrap();
                }
            });
        });
    }
}