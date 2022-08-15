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

    pub fn switch(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let event_handle = self.event.clone();
        let state_handle = self.state.clone();

        if let Some(route) = self.event.get_route() {
            match route {
                Route::Test => Test::new().view(ctx, event_handle, state_handle),
                Route::Test1 => Test1::new().view(ctx, event_handle, state_handle),
                Route::Test2 => Test2::new().view(ctx, event_handle, state_handle),
            }   
        } else {
            Test::new().view(ctx, event_handle, state_handle);
        }
    }

}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_state();
        self.switch(ctx, frame);
    }
}