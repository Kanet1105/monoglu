use crate::data::DataStates;
use chrono::{Duration, Utc};

pub struct Test2 {
    graph1: Graph1    
}

impl Test2 {
    pub fn new() -> Self {
        Self {
            graph1: Graph1::default(),
        }
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
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading); // 이후 해당 모든 ui style 을 변경
                        ui.label(format!("Test1 (x: {}, y: {})", data_states.d1.x.lock().unwrap(), data_states.d1.y.lock().unwrap()));
                    }

                    ui.separator();
                    self.graph1.option(ui);
                    self.graph1.show(ui);
                });
        });
    }
}

pub struct Graph1 {
    animate: bool,
    time: f64,
    proportional: bool,
    coordinates: bool,
    line_style: egui::widgets::plot::LineStyle,
}

impl Default for Graph1 {
    fn default() -> Self {
        Self {
            animate: !cfg!(debug_assertions),
            time: 0.0,
            proportional: true,
            coordinates: true,
            line_style: egui::widgets::plot::LineStyle::Solid,
        }
    }
}

impl Graph1 {
    fn option(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.animate, "Animate");
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input().unstable_dt.min(1.0 / 30.0) as f64;
        }

        let mut plot = egui::widgets::plot::Plot::new("graph1").legend(egui::widgets::plot::Legend::default());
        
        if self.proportional {
            plot = plot.view_aspect(1.0);
        }

        if self.coordinates {
            plot = plot.coordinates_formatter(egui::widgets::plot::Corner::LeftBottom, egui::widgets::plot::CoordinatesFormatter::default());
        }

        plot.show(ui, |plot_ui| {
            plot_ui.line(self.sin());
        }); 
    }
    
    fn sin(&self) -> egui::widgets::plot::Line {
        let time = self.time;
        egui::widgets::plot::Line::new(egui::widgets::plot::PlotPoints::from_explicit_callback(
            move |x| 0.5 * (2.0 * x).sin() * time.sin(),
            ..,
            512,
        ))
        .color(egui::Color32::from_rgb(150, 100, 100))
        .style(self.line_style)
        .name("sin")
    }
 
}