mod company_info;
mod etc1;
mod etc2;
mod exception;
mod home;
mod login;
mod plants;
mod servers;
mod tech_info;
mod user;


use crate::cell::Grid;
use exception::Exception;
use std::{collections::HashMap};

pub trait Tab {
    fn name(&self) -> &'static str;
    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub struct TabStates {
    tabs: HashMap<String, Box<dyn Tab>>,
    current_tab: String,
    grid: Grid,
}

impl TabStates {
    pub fn switch(&mut self, frame: &mut eframe::Frame) {
        self.current_tab = frame
            .info()
            .web_info
            .location
            .hash
            .to_owned();
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("main_tab_normal").show(ctx, |ui| {
            let browser_width = ui.ctx().available_rect().width();
            
            if browser_width <= 500.0 {
                self.mobile_view(ui, frame);
            } else if browser_width <= 1300.0 {
                self.normal_view(ui, frame);
            } else {
                self.wide_view(ui, frame);
            }
        });
            
        self.switch(frame);
        match self.tabs.get_mut(&self.current_tab) {
            Some(tab) => tab.view(ctx, frame),
            None => Exception::PageNotFound.view(ctx, frame),
        }
    }


    fn mobile_view(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let input_layout = egui::Layout::top_down(egui::Align::Center);
        
        ui.allocate_ui_with_layout(ui.available_size(), input_layout, |ui| {
            ui.horizontal(|ui| {
                ui.visuals_mut().button_frame = false;
        
                let list = [
                    ("/", "H"),                       // request to server again when an user click home button
                    ("#servers", "🖧"),
                    ("#plants", "v"),
                    ("#tech_info", "💡"),
                    ("#company_info", "🏢"),
                    ("#etc1", "E1"),
                    ("#etc2", "E2"),
                    ("#login", "🔧"),
                    ("#user", "👤")
                ];
                let mut selected_anchor = self.current_tab.clone();
                for (anchor, icon) in list {
                    if ui.add(
                        crate::monoglu_features::SelectableLabel::new(selected_anchor == anchor, icon)
                            .fill(egui::Color32::WHITE)
                    ).clicked()
                    {
                        selected_anchor = anchor.to_owned();
                        if frame.is_web() {
                            ui.output().open_url(format!("{}", anchor));
                        }
                    }
                }
                self.current_tab = selected_anchor;
            });
        });
    }
    
    fn normal_view(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.visuals_mut().button_frame = false;
        
                let list = [
                    ("/", "HOME"),                       // request to server again when an user click home button
                    ("#servers", "SERVER"),
                    ("#plants", "PLANTS"),
                    ("#tech_info", "TECH INFO"),
                    ("#company_info", "COMPANY INFO"),
                    ("#etc1", "ETC.1"),
                    ("#etc2", "ETC.2"),
                    ("#login", "SING IN"),
                    ("#user", "USER")
                ];
                let mut selected_anchor = self.current_tab.clone();
                for (anchor, icon) in list {
                    if ui.add(
                        crate::monoglu_features::SelectableLabel::new(selected_anchor == anchor, icon)
                            .fill(egui::Color32::WHITE)
                    ).clicked()
                    {
                        selected_anchor = anchor.to_owned();
                        if frame.is_web() {
                            ui.output().open_url(format!("{}", anchor));
                        }
                    }
                }
                self.current_tab = selected_anchor;
            });
        });
    }
    
    fn wide_view(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // ui.vertical_centered(|ui| {
        //     ui.horizontal(|ui| {
        //         ui.visuals_mut().button_frame = false;
        
        //         let list = [
        //             (0, "/", "HOME"),                       // request to server again when an user click home button
        //             (1, "#servers", "SERVER"),
        //             (2, "#plants", "PLANTS"),
        //             (3, "#tech_info", "TECH INFO"),
        //             (4, "#company_info", "COMPANY INFO"),
        //             (5, "#etc1", "ETC.1"),
        //             (6, "#etc2", "ETC.2"),
        //             (7, "#login", "SING IN"),
        //             (8, "#user", "USER")
        //         ];
        //         let mut selected_anchor = self.current_tab.clone();
        //         for (i, anchor, icon) in list {
        //             self.grid.get_cell(i, 0)
        //                 .add_contents(Box::new(|ui| {
        //                     if ui.add(
        //                         crate::monoglu_features::SelectableLabel::new(selected_anchor == anchor, icon)
        //                             .fill(egui::Color32::WHITE)
        //                     ).clicked()
        //                     {
        //                         selected_anchor = anchor.to_owned();
        //                         if frame.is_web() {
        //                             ui.output().open_url(format!("{}", anchor));
        //                         }
        //                     }        
        //                 }));
        //         }
        //         self.current_tab = selected_anchor;
        //     });
        // });
    }
    
}

impl Default for TabStates {
    fn default() -> Self {
        let mut tabs = HashMap::<String, Box<dyn Tab>>::new();
        tabs.insert("#company_info".to_string(), Box::new(company_info::CompanyInfo::new()));
        tabs.insert("#etc1".to_string(), Box::new(etc1::Etc1::new()));
        tabs.insert("#etc2".to_string(), Box::new(etc2::Etc2::new()));
        tabs.insert("".to_string(), Box::new(home::Home::new()));
        tabs.insert("#login".to_string(), Box::new(login::Login::new()));
        tabs.insert("#plants".to_string(), Box::new(plants::Plants::new()));
        tabs.insert("#servers".to_string(), Box::new(servers::Servers::new()));
        tabs.insert("#tech_info".to_string(), Box::new(tech_info::TechInfo::new()));
        tabs.insert("#user".to_string(), Box::new(user::User::new()));

        Self {
            tabs,
            current_tab: "/".to_string(),
            grid: Grid::new("main_tab", 1, 9),
        }
    }
}
