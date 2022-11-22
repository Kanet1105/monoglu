use crate::prelude::*;
use serde::Deserialize;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

#[derive(Deserialize)]
pub struct Server {
    pub ip_v4: String,
    pub ip_v6: Option<String>,
    pub port: u16,
    pub max_concurrency: u16,
}

impl Server {
    pub fn endpoint_v4(&self) -> Result<SocketAddr> {
        let ip = self.ip_v4.parse::<Ipv4Addr>()?;
        Ok(SocketAddr::V4(SocketAddrV4::new(ip, self.port)))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            ip_v4: "127.0.0.1".to_string(),
            ip_v6: None,
            port: 5000,
            max_concurrency: 1000,
        }
    }
}

#[test]
fn parser_1() {
    let ip = "127.0.0.1".to_string();
    let ip_v4: Vec<u8> = ip
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    println!("{:?}", ip_v4);
}

#[test]
fn parser_2() {
    let ip = "127.0.0.1".to_string();
    let ip_v4 = ip.parse::<Ipv4Addr>().unwrap();
    println!("{:?}", ip_v4);
}
