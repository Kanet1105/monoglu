use crate::prelude::*;

pub struct Application {
    event: Event,
    state: State,
}

impl Application {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            event: EventHandle::new(20, 5),
            state: StateHandle::new(),
        }
    }

    pub fn update_state(&self) {
        self.event.run_events().unwrap();
    }

    pub fn view(&self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        switch(ctx, frame, self.event.clone(), self.state.clone());
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_state();
        self.view(ctx, frame);
    }
}