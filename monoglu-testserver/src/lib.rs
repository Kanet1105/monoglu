mod crypto;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub use tracing::{error, info};
}

use crate::prelude::*;
use ring::rand::{SecureRandom, SystemRandom};
use std::{io::ErrorKind, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, io::AsyncWriteExt};

/// ### REQUIREMENTS
/// * CMAKE
/// * NASM
#[tokio::main]
pub async fn run_server() -> Result<()> {
    // crypto::generate_pem("./cert", "temp")?;
    let mut socket = UdpSocket::bind("127.0.0.1:5000").await?;

    // https://github.com/cloudflare/quiche/blob/master/quiche/examples/server.rs#L78
    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
    config.load_cert_chain_from_pem_file("./cert/temp_cert.pem")?;
    config.load_priv_key_from_pem_file("./cert/temp_key.pem")?;
    config.set_application_protos(&[b"hq-interop", b"hq-29", b"hq-28", b"hq-27", b"http/0.9"])?;

    let rng = ring::rand::SystemRandom::new();
    let connection_id_seed =
        ring::hmac::Key::generate(ring::hmac::HMAC_SHA256, &rng).unwrap();

    let mut socket_buffer = Vec::<u8>::with_capacity(1024);
    loop {
        match socket.recv_from(&mut socket_buffer).await {
            Ok((length, remote_address)) => {
                let header = quiche::Header::from_slice(
                    &mut socket_buffer[0..length],
                    quiche::MAX_CONN_ID_LEN,
                )?;
                let tag = ring::hmac::sign(
                    &connection_id_seed,
                    &header.dcid,
                );
                let connection_id = &tag
                    .as_ref()[..quiche::MAX_CONN_ID_LEN]
                    .to_vec()
                    .into();
                let connection = quiche::accept(
                    connection_id,
                    None,
                    socket.local_addr()?,
                    remote_address.clone(),
                    &mut config,
                )?;
                handle_connection(connection).await;
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    continue;
                } else {
                    error!("{}", e);
                    panic!();
                }
            }
        };
    }
}

pub async fn handle_connection(connection: quiche::Connection) {
    if connection.is_established() {
        info!("Connection established..");
    }
}

#[tokio::main]
pub async fn run_client() -> Result<()> {
    use tokio::time::{sleep, Duration};

    let mut socket = UdpSocket::bind("127.0.0.1:0").await?;
    let server_addr = "127.0.0.1:5000".parse::<SocketAddr>()?;

    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
    config.verify_peer(false);
    config.set_application_protos(&[b"hq-interop", b"hq-29", b"hq-28", b"hq-27", b"http/0.9"])?;

    let mut scid = [0; quiche::MAX_CONN_ID_LEN];
    SystemRandom::new().fill(&mut scid[..]).unwrap();
    let scid = quiche::ConnectionId::from_ref(&scid);

    let mut connection = quiche::connect(
        Some("TestServer"), 
        &scid, 
        socket.local_addr()?,
        server_addr,
        &mut config,
    )?;

    let mut recv_buffer = Vec::<u8>::with_capacity(64);
    tokio::spawn(async move {
        loop {
            
        }
    });

    let mut send_buffer = Vec::<u8>::with_capacity(64);
    loop {
        for i in 0..100 {
            send_buffer.write_u64(i as u64);
            let (write, send_info) = match connection.send(&mut send_buffer) {
                Ok(v) => v,
                Err(e) => {
                    error!("{}", e);
                    break;
                },
            };
            sleep(Duration::from_millis(500)).await;
            send_buffer.clear();
        }
    }
    Ok(())
}
