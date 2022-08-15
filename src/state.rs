use crate::prelude::*;

pub type State = Arc<Mutex<StateHandle>>;

pub struct StateHandle {
    pub counter: i32,
    pub counter1: i32,
    pub counter2: i32,
}

impl StateHandle {
    pub fn new() -> State {
        Mutex::new(Self {
            counter: 0,
            counter1: 1,
            counter2: 2,
        }).into()
    }
}