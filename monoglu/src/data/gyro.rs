use super::Interface;
use rand::{prelude::*, distributions::Uniform};

pub struct Gyro {
    roll: f32,
    pitch: f32,
    yaw: f32,
}

impl Gyro {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, 180.0);
        Self {
            roll: uniform.sample(&mut rng),
            pitch: uniform.sample(&mut rng),
            yaw: uniform.sample(&mut rng),
        }
    }
}

impl std::fmt::Display for Gyro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Roll: {}\nPitch: {}\nYaw: {}", self.roll, self.pitch, self.yaw)
    }
}

impl Interface for Gyro {
    fn fetch(&mut self) {
        unimplemented!();
    }
}

#[test]
fn test_fetch() {
    let mut sensor = Gyro::new();
    sensor.fetch();
    println!("{}", sensor);
}
