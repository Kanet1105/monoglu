use crate::data::{*, distance::Distance, gyro::Gyro};

pub struct Controller {
    polling_rate: u64, // milliseconds
    distance: Distance,
    gyro: Gyro,
    eps: f64,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            polling_rate: 100,
            distance: Distance::new(),
            gyro: Gyro::new(),
            eps: 0.5,
        }
    }

    pub fn poll_sensor_values(&mut self) {
        self.distance.fetch();
        self.gyro.fetch();
    }

    pub fn pid(&mut self, target_roll: f64, target_pitch: f64, target_yaw: f64) {
        let p = 0.05;
        let d = 0.001;

        let (roll, pitch, yaw) = self.gyro.get_current();
        let mut err_roll = target_roll - roll;
        let mut err_pitch = target_pitch - pitch;
        let mut err_yaw = target_yaw - yaw;
        println!("{:?} {:?} {:?}", err_roll, err_pitch, err_yaw);

        while err_roll.abs() >= self.eps || err_pitch.abs() >= self.eps || err_yaw.abs() >= self.eps {
            let (roll, pitch, yaw) = self.gyro.get_current();
            err_roll = target_roll - roll;
            err_pitch = target_pitch - pitch;
            err_yaw = target_yaw - yaw;
            let (d_roll, d_pitch, d_yaw) = self.gyro.get_differences();

            // PD controller
            let roll_offset = (err_roll * p) - (d_roll * d);
            let pitch_offset = (err_pitch * p) - (d_pitch * d);
            let yaw_offset = (err_yaw * p) - (d_yaw * d);
            self.gyro.update_gyro_values(roll_offset, pitch_offset, yaw_offset);
            println!("{}", self.gyro);
            std::thread::sleep(std::time::Duration::from_millis(self.polling_rate));
        }
    }
}
