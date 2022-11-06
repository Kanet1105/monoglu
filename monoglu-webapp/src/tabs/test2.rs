use crate::data::DataStates;

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
                
                ui.vertical(|ui| {
                    if ui.button("Test2 a increment").clicked() {
                        
                        let mut data_guard = data1.lock().unwrap();
                        *data_guard += 0.1;
                    }
                    if ui.button("Text2 a decrement").clicked() {
                        let mut data_guard = data2.lock().unwrap();
                        *data_guard -= 0.1;
                    }

                    ui.separator();

                    ui.label(format!("Test1 - (x: {}, y: {})", data_states.d1.x.lock().unwrap(), data_states.d1.y.lock().unwrap()));
                });
        });
    }
}