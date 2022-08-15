use crate::prelude::*;

pub enum EventType {
    Update,
    Route,
}

pub type Event = Arc<EventHandle>;
type EventQueue = Mutex<VecDeque<Box<dyn FnOnce() + 'static>>>;
type RouterQueue = Mutex<VecDeque<Box<dyn FnOnce() + 'static>>>;

pub struct EventHandle {
    event_capacity: usize,
    router_capacity: usize,
    event: EventQueue,
    router: RouterQueue,
}

impl EventHandle {
    pub fn new(event_len: usize, router_len: usize) -> Event {
        let event = VecDeque::<Box<dyn FnOnce() + 'static>>::with_capacity(event_len);
        let router = VecDeque::<Box<dyn FnOnce() + 'static>>::with_capacity(router_len);

        Self {
            event_capacity: event_len,
            router_capacity: router_len,
            event: event.into(),
            router: router.into(),
        }.into()
    }

    pub fn push<F>(&self, event: EventType, callback: F) -> Result<(), Box<dyn std::error::Error>> where 
        F: FnOnce() + 'static,
    {
        match event {
            EventType::Update => {
                let mut event_guard = self.event.lock().unwrap();
                if &event_guard.len() == &self.event_capacity {
                    return Err(format!("event queue (capacity = {}) overflow..", &self.event_capacity).into())
                } else {
                    event_guard.push_back(Box::new(callback));
                    return Ok(())
                }
            },
            EventType::Route => {
                let mut router_guard = self.router.lock().unwrap();
                if &router_guard.len() == &self.router_capacity {
                    let _ = router_guard.pop_front().unwrap();
                }
                router_guard.push_back(Box::new(callback));
                return Ok(())
            },
        }
    }

    pub fn run_events(&self) {
        let mut queue_guard = self.event.lock().unwrap();
        while let Some(event) = queue_guard.pop_front() {
            event();
        }
    }

    // pub fn switch_route(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut router_guard = self.router.lock().unwrap();
    //     if let Some(route) = router_guard.back() {move || {
    //         route();
    //         return Ok(())
    //     }} else {
    //         return Err("empty router buffer..".into())
    //     };
    // }
}

#[test]
fn test_function() {
    let event = EventHandle::new(20, 10);
    
    for n in 0..20 {
        event.push(EventType::Update, move || {
            dbg!(n);
        }).unwrap();
    }

    event.run_events();
}