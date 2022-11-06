use std::fmt::format;

use crate::data::DataStates;

pub struct Test1;

impl Test1 {
    pub fn new() -> Self {
        Self
    }
}

impl super::Tab for Test1 {
    fn name(&self) -> &'static str {
        "test1"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, data_states: &crate::data::DataStates) {
        let data1 = data_states.d1.x.clone();
        let data2 = data_states.d1.x.clone();

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical(|ui| {
                if ui.button("Test1 x increment").clicked() {
                    DataStates::dispatch(Box::new(data1), move |data1| {
                        let mut data_guard = data1.lock().unwrap();
                        *data_guard += 1;
                    });
                };
    
                if ui.button("Test1 x decrement").clicked() {
                    DataStates::dispatch(Box::new(data2), move |data2| {
                        let mut data_guard = data2.lock().unwrap();
                        *data_guard -= 1;
                    });
                }
    
                ui.separator();
                
                ui.label(format!("Test2 (a: {}, b: {})", data_states.d2.a.lock().unwrap(), data_states.d2.b.lock().unwrap()));
            });
        });
    }
}