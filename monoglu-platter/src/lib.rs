mod configuration;

pub mod prelude {
    pub type Exception = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Exception>;

    pub use crate::run_server;
    pub use tracing::{error, info};
}

use crate::prelude::*;
use tokio::{
    net::{TcpListener, TcpStream}, 
    io::AsyncWriteExt
};

pub async fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7627").await?;
    
    loop {
        let (stream, _address) = listener.accept().await?;
        connection_handler(stream).await?;
    }
}

pub async fn connection_handler(mut stream: TcpStream) -> Result<()> {
    
    stream.shutdown().await?;
    Ok(())
}
