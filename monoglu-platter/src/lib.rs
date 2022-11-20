mod configuration;
mod database;

pub mod prelude {
    pub type Exception = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Exception>;

    pub use crate::{
        run_server, 
        user_input, 
        configuration::Config,
        database::{DatabaseBuilder, Database},
    };
    pub use tracing::{error, info};
}

use crate::prelude::*;
use bytes::{BufMut, BytesMut};
use std::{io::ErrorKind};
use tokio::{
    io::{copy, split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn run_server() -> Result<()> {
    let config = Config::from_file("./Config.toml")?;
    let listener = TcpListener::bind(config.server.address()).await?;
    // TODO(): Create a database instance.
    // let db = DatabaseBuilder::InfluxDB::init();

    loop {
        let (stream, _address) = listener.accept().await?;
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

pub fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
