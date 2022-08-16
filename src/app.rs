use crate::prelude::*;

pub struct Application {
    event: Event,
    state: State,
}

impl Application {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            event: Event::new(20, 5),
            state: State::new(),
        }
    }

    pub fn view(&self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update states.
        self.event.run_events().unwrap();

        // Fetch the requested page to view. It refers to the final element
        // of the router queue in EventHandle.
        switch(ctx, frame, &self.event, &self.state);
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.view(ctx, frame);
    }
}