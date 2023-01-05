use super::Interface;
use rand::{distributions::Uniform, prelude::*};

pub struct Distance {
    in_cm: f64,
}

impl Distance {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, 100.0);
        Self {
            in_cm: uniform.sample(&mut rng),
        }
    }

    pub fn update_distance(&mut self, dist: f64) {
        self.in_cm = dist;
    }
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Distance in cm: {}", self.in_cm)
    }
}

impl Interface for Distance {
    fn fetch(&mut self) {
        unimplemented!();
    }
}

#[test]
fn test_fetch() {
    let mut sensor = Distance::new();
    sensor.fetch();
    println!("{}", sensor);
}
