use serde::Deserialize;

#[derive(Deserialize)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    pub max_concurrency: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 5000,
            max_concurrency: 1000,
        }
    }
}
