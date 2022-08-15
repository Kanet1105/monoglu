use crate::prelude::*;

pub struct Navigator;

impl Component for Navigator {
    type Router = Route;

    fn new() -> Self {
        Self {}    
    }

    fn view(&self, ctx: &egui::Context, event: crate::prelude::Event, state: crate::prelude::State) {
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let test = ui.button("Test");
                if test.clicked() {
                    event.route(Self::Router::Test).unwrap();
                };

                let test1 = ui.button("Test1");
                if test1.clicked() {
                    event.route(Self::Router::Test1).unwrap();
                };

                let test2 = ui.button("Test2");
                if test2.clicked() {
                    event.route(Self::Router::Test2).unwrap();
                };
            });
        });
    }
}