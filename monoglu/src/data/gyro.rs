use super::Interface;
use rand::{distributions::Uniform, prelude::*};

pub struct Gyro {
    prev_roll: f64,
    roll: f64,
    prev_pitch: f64,
    pitch: f64,
    prev_yaw: f64,
    yaw: f64,
}

impl Gyro {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new(0.0, 180.0);
        let init_roll = uniform.sample(&mut rng);
        let init_pitch = uniform.sample(&mut rng);
        let init_yaw = uniform.sample(&mut rng);

        Self {
            prev_roll: init_roll,
            roll: init_roll,
            prev_pitch: init_pitch,
            pitch: init_pitch,
            prev_yaw: init_yaw,
            yaw: init_yaw,
        }
    }

    pub fn get_current(&self) -> (f64, f64, f64) {
        (self.roll, self.pitch, self.yaw)
    }

    pub fn get_differences(&self) -> (f64, f64, f64) {
        ((self.prev_roll - self.roll).abs(), (self.prev_pitch - self.pitch).abs(), (self.prev_yaw - self.yaw).abs())
    }

    pub fn update_gyro_values(&mut self, roll_offset: f64, pitch_offset: f64, yaw_offset: f64) {
        self.prev_roll = self.roll;
        self.roll += roll_offset;
        self.prev_pitch = self.pitch;
        self.pitch += pitch_offset;
        self.prev_yaw = self.yaw;
        self.yaw += yaw_offset;
    }
}

impl std::fmt::Display for Gyro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Roll: {}\nPitch: {}\nYaw: {}\n============================",
            self.roll, self.pitch, self.yaw
        )
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
