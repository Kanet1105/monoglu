use crate::cell::Grid;
use crate::monoglu_features::Button;

pub struct Servers {
    show_machine_num: usize,
    // dialog_grid: Grid,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            show_machine_num: 0,
            // dialog_grid: Gird::new("dialog_grid", 1, 1),
        }
    }

    fn mobile_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("mobile_servers_dialog").show(ctx, |ui| {
            let grid = Grid::new("mobile_servers_dialog_grid", 5, 5);

        });

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Servers");
        });
    }

    fn normal_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("normal_server_dialog").show(ctx, |ui| {
            //--------------
            ui.horizontal(|ui|{
                if ui.add(Button::new(
                    egui::RichText::new("🖧")   
                        .text_style(egui::TextStyle::Name("emoji_big".into()))
                ))
                .on_hover_text("leader manager")
                .clicked() {
                    self.show_machine_num += 1;
                }
                
                ui.separator();
    
                if ui.add(Button::new(
                    egui::RichText::new("🖧")   
                        .text_style(egui::TextStyle::Name("emoji_big".into()))
                ))
                .on_hover_text("leader manager")
                .clicked() {
                    self.show_machine_num += 1;
                }        
                
                ui.separator();
    
                if ui.add(Button::new(
                    egui::RichText::new("🖧")   
                        .text_style(egui::TextStyle::Name("emoji_big".into()))
                ))
                .on_hover_text("leader manager")
                .clicked() {
                    self.show_machine_num += 1;
                }        
            });

            //-----------------------------------

            // let mut dialog_grid = Grid::new("dailog_grid", 1, 12);
            // dialog_grid.show(ctx);
            
            // dialog_grid.get_cell(0, 0)
            //     .add_contents(Box::new(|ui| {
            //         if ui.add(Button::new(
            //             egui::RichText::new("🖧")   
            //                 .text_style(egui::TextStyle::Name("emoji_big".into()))
            //         ))
            //         .on_hover_text("leader manager")
            //         .clicked() {
            //             self.show_machine_num += 1;
            //         }        
            //     }));

            //     dialog_grid.get_cell(0, 2)
            //     .add_contents(Box::new(|ui| {
            //         if ui.add(Button::new(
            //             egui::RichText::new("🖧")   
            //                 .text_style(egui::TextStyle::Name("emoji_big".into()))
            //         ))
            //         .on_hover_text("follower manager 1")
            //         .clicked() {
            //             self.show_machine_num += 1;
            //         }        
            //     }));            

            //     dialog_grid.get_cell(0, 5)
            //     .add_contents(Box::new(|ui| {
            //         if ui.add(Button::new(
            //             egui::RichText::new("🖧")   
            //                 .text_style(egui::TextStyle::Name("emoji_big".into()))
            //         ))
            //         .on_hover_text("service worker 1")
            //         .clicked() {
            //             self.show_machine_num += 1;
            //         }        
            //     }));            
        });
        
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label(format!("Servers {}", self.show_machine_num));
        });
    }

    fn wide_view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.label("Servers");
        });
    }
}

impl super::Tab for Servers {
    fn name(&self) -> &'static str {
        "Servers"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let browser_width = ctx.available_rect().width();

        if browser_width <= 500.0 {
            self.mobile_view(ctx, frame);
        } else if browser_width <= 1300.0 {
            self.normal_view(ctx, frame);
        } else {
            self.wide_view(ctx, frame);
        }
    }
}