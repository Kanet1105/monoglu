use crate::prelude::*;

pub type Event = Arc<EventHandle>;
type EventQueue = Mutex<VecDeque<Box<dyn FnOnce() + 'static>>>;
type RouterQueue = Mutex<VecDeque<Route>>;

pub struct EventHandle {
    event_capacity: usize,
    router_capacity: usize,
    event: EventQueue,
    router: RouterQueue,
}

impl EventHandle {
    pub fn new(event_cap: usize, router_cap: usize) -> Event {
        let event = VecDeque::<Box<dyn FnOnce() + 'static>>::with_capacity(event_cap);
        let router = VecDeque::<Route>::with_capacity(router_cap);

        Self {
            event_capacity: event_cap,
            router_capacity: router_cap,
            event: event.into(),
            router: router.into(),
        }.into()
    }

    pub fn push<F>(&self, callback: F) -> Result<(), Box<dyn std::error::Error>> where 
        F: FnOnce() + 'static,
    {
        let mut event_guard = self.event.lock().unwrap();
        if &event_guard.len() == &self.event_capacity {
            return Err(format!("event queue (capacity = {}) overflow..", &self.event_capacity).into())
        } else {
            event_guard.push_back(Box::new(callback));
            return Ok(())
        }
    }

    pub fn route(&self, route: Route) -> Result<(), Box<dyn std::error::Error>> {
        let mut router_guard = self.router.lock().unwrap();
        if &router_guard.len() == &self.router_capacity {
            let _ = router_guard.pop_front().unwrap();
        }
        router_guard.push_back(route);
        
        Ok(())
    }

    pub fn get_route(&self) -> Option<Route> {
        let router_guard = self.router.lock().unwrap();
        match router_guard.back() {
            Some(route) => Some(route.clone()),
            None => None,
        }
    }

    pub fn run_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_guard = self.event.lock().unwrap();
        while let Some(event) = event_guard.pop_front() {
            event();
        }

        Ok(())
    }
}

#[test]
fn test_function() {
    let event = EventHandle::new(20, 10);
    
    for n in 0..20 {
        event.push(move || {
            dbg!(n);
        }).unwrap();
    }

    event.run_events().unwrap();
}