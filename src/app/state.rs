use std::time::{Duration, Instant};
use std::sync::Arc;

pub type AppState = Arc<State>;

pub fn new_state() -> AppState {
    Arc::new(State::new())
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct State {
    last_rendered: f64,
    fps: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            last_rendered: 0.0, 
            fps: 0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}