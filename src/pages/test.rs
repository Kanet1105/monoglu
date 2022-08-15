use crate::prelude::*;

pub struct Test;

impl Test {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Test {
    fn view(&self, ctx: &egui::Context, event: Event, state: State) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let counter = {
                let state_guard = state.lock().unwrap();
                state_guard.counter.to_string()
            };
            ui.label(counter);

            ui.horizontal(|ui| {
                let event_handle = event.clone();
                let inc = state.clone();
                let dec = state.clone();

                let increment = ui.button("increment");
                if increment.clicked() {
                    event_handle.push(EventType::Update, move || {
                        let mut state_guard = inc.lock().unwrap();
                        state_guard.counter += 1;
                    }).unwrap();
                };

                let decrement = ui.button("decrement");
                if decrement.clicked() {
                    event_handle.push(EventType::Update, move || {
                        let mut state_guard = dec.lock().unwrap();
                        state_guard.counter -= 1;
                    }).unwrap();
                }
            });
        });
    }
}