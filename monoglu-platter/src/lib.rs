mod data;
mod event;

pub mod prelude {
    pub type Exception = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Exception>;

    pub use crate::{
        run_server,
        data::{Config, Storage},
        event::dispatch,
    };
    pub use tracing::{error, info};
}

use crate::prelude::*;
use bytes::{BufMut, BytesMut};
use rand::prelude::*;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    io::ErrorKind, 
    net::SocketAddr, 
};
use tokio::{
    io::{copy, split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, UdpSocket},
    sync::{mpsc, Mutex},
};

use tokio::time::{sleep, Duration, Instant};

pub async fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7627").await?;

    loop {
        let (stream, address) = listener.accept().await?;
        connection_handler(stream).await?;
    }
}

pub async fn connection_handler(mut stream: TcpStream) -> Result<()> {
    let mut buffer = BytesMut::with_capacity(1024);
    
    tokio::spawn(async move {
        loop {
            stream.readable().await.unwrap();

            match stream.try_read_buf(&mut buffer) {
                Ok(0) => {
                    error!("stream's read half is closed.");
                    break;
                }
                Ok(n) => {
                    info!("received {} bytes.", n);
                    
                    continue;
                },
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                },
                Err(e) => {
                    error!("{}", e);
                    break;
                }
            }
        }
        stream.shutdown().await.unwrap();
    });

    Ok(())
}

pub fn authenticate_node() {}

pub fn generate_key() {
    let mut hasher = Sha256::new();
    hasher.update("Hello, node!");
    let key: String = format!("{:x}", hasher.finalize());
}
