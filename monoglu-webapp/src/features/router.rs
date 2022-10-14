use crate::pages::View;
use std::collections::VecDeque;

pub struct Router<T: View> {
    history: VecDeque<Box<T>>,
    home: Box<T>,
}

impl<T: View> Router<T> {
    pub fn new(capacity: usize, home: T) -> Self {
        Self {
            history: VecDeque::with_capacity(capacity),
            home: Box::new(home),
        }
    }

    pub fn switch(&mut self, page: T) {
        self.history.push_back(Box::new(page));
    }

    pub fn render(&self, ctx: &egui::Context) {
        match self.history.back() {
            Some(page) => page.show(ctx),
            None => self.home.show(ctx),
        }
    }
}
