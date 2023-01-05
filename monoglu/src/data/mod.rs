pub mod distance;
pub mod gyro;

use std::fmt::Display;

pub trait Interface: Display {
    fn fetch(&mut self) {}
    // fn print(&self) {}
    // fn serialize(&self) {}
    // fn deserialize(&self) {}
}
