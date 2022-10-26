use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
};


pub struct Servers {
    pub seeds: Rc<RefCell<BTreeMap<String, Seed>>>,
    pub services: Rc<RefCell<BTreeMap<String, Service>>>,
}

impl Servers {
    pub fn new () -> Self {
        Self {
            seeds: Rc::new(RefCell::new(BTreeMap::<String, Seed>::new())),
            services: Rc::new(RefCell::new(BTreeMap::<String, Service>::new())),
        }
    }

    pub fn insert_seed(&mut self, id: String, seed: Seed) {
        self.seeds.borrow_mut().insert(id, seed);
    }

    pub fn remove_seed(&mut self, id: &String) {
        match self.seeds.borrow_mut().remove(id) {
            Some(_) => log::info!("{} is removed successfully", id),
            None => log::info!("{} is not exist.", id),
        }
    }

    pub fn insert_service(&mut self, id: String, service: Service) {
        self.services.borrow_mut().insert(id, service);
    }

    pub fn remove_service(&mut self, id: &String) {
        match self.services.borrow_mut().remove(id) {
            Some(_) => log::info!("{} is removed successfully", id),
            None => log::info!("{} is not exist.", id),
        }
    }
}


pub struct Seed {
    pub role: Role,
    pub data1: f32,
    pub data2: f32,
    pub data3: f32,
}

impl Seed {
    pub fn new(
        id: String, 
        role: Role, 
        data1: f32, 
        data2: f32, 
        data3: f32
    ) -> (String, Self) {
        (
            id,
            Self {
                role,
                data1,
                data2,
                data3,
            }
        )
    }
}


pub struct Service {
    pub info1: f32,
    pub info2: f32,
    pub info3: f32,
}

impl Service {
    pub fn new(
        id: String,
        info1: f32, 
        info2: f32, 
        info3: f32
    ) -> (String, Self) {
        (
            id,
            Self {
                info1,
                info2,
                info3,
            }
        )
    }
}

pub enum Role {
    Leader, Follower,
}