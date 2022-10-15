use crate::pages::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum Routes {
    Login,
}

impl Routes {
    pub fn render(&self, ctx: &egui::Context) {
        match self {
            Self::Login => LoginPage::show(ctx),
        }
    }
}

pub struct Router {
    history: VecDeque<Routes>,
    home: Routes,
}

impl Router {
    pub fn new(capacity: usize, home: Routes) -> Self {
        Self {
            history: VecDeque::with_capacity(capacity),
            home,
        }
    }

    pub fn switch(&mut self, page: Routes) {
        if self.history.len() == self.history.capacity() {
            self.history.pop_front();
        }
        self.history.push_back(page);
    }

    pub fn render(&self, ctx: &egui::Context) {
        match self.history.back() {
            Some(route) => {

            },
            None => ,
        }
    }
}
