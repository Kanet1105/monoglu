use crate::data::DataStates;
use chrono::{Duration, Utc};
pub struct Test2 {}

impl Test2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Tab for Test2 {
    fn name(&self) -> &'static str {
        "test2"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, data_states: &crate::data::DataStates) {
        egui::CentralPanel::default().show(ctx, |ui| {
                let data1 = data_states.d2.a.clone();
                let data2 = data_states.d2.a.clone();

                // let time = data_states.d1.event_time.clone();
                let occurance = data_states.d1.occurance.clone();
                
                ui.vertical(|ui| {
                    // if ui.button("Test2 a increment").clicked() {
                        
                    //     let mut data_guard = data1.lock().unwrap();
                    //     *data_guard += 0.1;
                    // }
                    // if ui.button("Text2 a decrement").clicked() {
                    //     let mut data_guard = data2.lock().unwrap();
                    //     *data_guard -= 0.1;
                    // }

                    // ui.separator();

                    if  *occurance.borrow() {
                        let duration = (Utc::now() - *data_states.d1.event_time.borrow()).num_seconds();
                        if duration >= 3 as i64 {
                            let _ = occurance.replace(false);
                        }
                        ui.spinner();
                    } else {
                        ui.label(format!("Test1 (x: {}, y: {})", data_states.d1.x.lock().unwrap(), data_states.d1.y.lock().unwrap()));
                    }
                });
        });
    }
}