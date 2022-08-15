use crate::prelude::*;

/// When you add a new page module, it is recommended to add the same name of 
/// the page component struct inside the Route enum to avoid confusion.
#[derive(Clone)]
pub enum Route {
    Test,
    Test1,
    Test2,
}

/// redirect the user to the new page with updated state and event handle.
pub fn switch(ctx: &egui::Context, _frame: &mut eframe::Frame, event: Event, state: State) {
    if let Some(route) = event.get_route() {
        match route {
            Route::Test => page::Test::new().view(ctx, event, state),
            Route::Test1 => page::Test1::new().view(ctx, event, state),
            Route::Test2 => page::Test2::new().view(ctx, event, state),
        }   
    } else {
        // redirect to the home page when the router queue is empty.
        page::Test::new().view(ctx, event, state);
    }
}