mod data;
mod event;

pub mod prelude {
    pub type Exception = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Exception>;

    pub use crate::{
        run_server,
        data::AppData,
        event::dispatch,
    };
    pub use tracing::{error, info};
}

use crate::prelude::*;
use rand::prelude::*;
use tokio::{
    net::{TcpListener, TcpStream, UdpSocket}, 
    io::AsyncWriteExt,
    sync::mpsc,
};

use tokio::time::{sleep, Duration, Instant};

pub async fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7627").await?;
    let app_data = AppData::new();

    let (tx, mut rx) = mpsc::channel(10);
    dispatch(app_data.clone(), |data: AppData| async move {
        loop {
            match rx.recv().await {
                Some(value) => {
                    info!("{}", value);
                },
                None => {
                    continue;
                }
            }
        }
    });

    loop {
        // tx.send("Hello, world!").await?;
        // sleep(Duration::from_millis(1000)).await;
        let (stream, _address) = listener.accept().await?;
        connection_handler(stream).await?;
    }
}

pub async fn connection_handler(mut stream: TcpStream) -> Result<()> {
    stream.shutdown().await?;
    Ok(())
}
