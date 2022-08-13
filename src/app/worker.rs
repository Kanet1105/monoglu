use std::thread;
use std::time::Duration;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Worker {
    counter: i32,
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            counter: 0,
        }
    }
}

impl Worker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        loop {
            thread::sleep(Duration::from_secs(2));
            dbg!(&self.counter);
        }
        Ok(())
    }
}