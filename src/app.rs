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

    pub fn run(&mut self) {
        self.event.run_events();
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.run();
        Test::new().view(ctx, self.event.clone(), self.state.clone());
    }
}