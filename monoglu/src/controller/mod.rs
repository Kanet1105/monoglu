use crate::{
    data::*,
    message::Message,
};
use std::{
    collections::HashMap,
    time::Duration,
    sync::mpsc,
    thread,
};

pub struct Controller {
    sensors: HashMap<String, Box<dyn Interface>>,
    polling_rate: u64, // milliseconds
    receiver: mpsc::Receiver<Message>,
}

impl Controller {
    pub fn new(rx: mpsc::Receiver<Message>) -> Self {
        let mut sensors = HashMap::<String, Box<dyn Interface>>::new();
        sensors.insert("dist".into(), Box::new(distance::Distance::new()));
        sensors.insert("gyro".into(), Box::new(gyro::Gyro::new()));
        Self {
            sensors,
            polling_rate: 0,
            receiver: ,
        }
    }

    pub fn poll_sensor_values(&mut self) {
        for (_, sensor) in &mut self.sensors {
            sensor.fetch();
        }
    }

    pub fn init(&mut self) {
        loop {
            thread::sleep(Duration::from_millis(self.polling_rate));
        }
    }
}
