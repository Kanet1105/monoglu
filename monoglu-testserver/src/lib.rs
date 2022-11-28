mod crypto;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub use tracing::{error, info};
}

use crate::prelude::*;
use bytes::BufMut;
use ring::rand::{SecureRandom, SystemRandom};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

static MAX_DATAGRAM_SIZE: usize = 1350;

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
    config.set_max_idle_timeout(5000);
    config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_initial_max_data(10_000_000);
    config.set_initial_max_stream_data_bidi_local(1_000_000);
    config.set_initial_max_stream_data_bidi_remote(1_000_000);
    config.set_initial_max_stream_data_uni(1_000_000);
    config.set_initial_max_streams_bidi(100);
    config.set_initial_max_streams_uni(100);
    config.set_disable_active_migration(true);
    config.enable_early_data();

    let rng = ring::rand::SystemRandom::new();
    let connection_id_seed = ring::hmac::Key::generate(ring::hmac::HMAC_SHA256, &rng).unwrap();

    let mut pool = HashMap::<quiche::ConnectionId<'static>, Arc<Mutex<quiche::Connection>>>::new();

    let mut recv_buffer = vec![0; MAX_DATAGRAM_SIZE];
    // let mut output_buffer = vec![0; MAX_DATAGRAM_SIZE];
    loop {
        let (len, remote_addr) = socket.recv_from(&mut recv_buffer).await?;
        let header = quiche::Header::from_slice(&mut recv_buffer[0..len], quiche::MAX_CONN_ID_LEN)?;
        let connection_id: quiche::ConnectionId = ring::hmac::sign(&connection_id_seed, &header.dcid)
            .as_ref()[..quiche::MAX_CONN_ID_LEN]
            .to_vec()
            .into();
        let stream = if !pool.contains_key(&connection_id) {
            // Check if the packet type is 'Initial'.
            if header.ty != quiche::Type::Initial {
                error!("Packet type is not 'Initial'.");
                continue;
            }

            // Version negotiation.
            // if quiche::version_is_supported(header.version) {
            //     info!("Negotiating the version.");
            //     let n = quiche::negotiate_version(&header.scid, &header.dcid, &mut output_buffer)?;
            //     if let Err(e) = socket.send_to(&output_buffer[0..n], &remote_addr).await {
            //         if e.kind() == std::io::ErrorKind::WouldBlock {
            //             error!("send_to() would block.");
            //             break;
            //         }
            //         panic!();
            //     }
            //     continue;
            // }

            let connection = quiche::accept(
                &connection_id,
                None,
                socket.local_addr()?,
                remote_addr,
                &mut config,
            )?;
            info!("Connected!");
            let stream = Arc::new(Mutex::new(connection));
            pool.insert(connection_id.clone(), stream.clone());
            stream
        } else {
            let stream = Arc::clone(pool.get(&connection_id).unwrap());
            stream
        };
        tokio::spawn(handle_stream(stream));
    }
}

pub async fn handle_stream(stream: Arc<Mutex<quiche::Connection>>) {
    let mut recv_buffer = vec![0; MAX_DATAGRAM_SIZE];
    loop {
        let mut stream_guard = stream.lock().await;
        let (len, is_fin) = match stream_guard.stream_recv(0, &mut recv_buffer) {
            Ok(v) => v,
            Err(_) => {
                continue;
            },
        };
        stream_guard.stream_send(0, &recv_buffer[0..len], is_fin).unwrap();
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
    config.set_max_idle_timeout(5000);
    config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_initial_max_data(10_000_000);
    config.set_initial_max_stream_data_bidi_local(1_000_000);
    config.set_initial_max_stream_data_bidi_remote(1_000_000);
    config.set_initial_max_streams_bidi(100);
    config.set_initial_max_streams_uni(100);
    config.set_disable_active_migration(true);

    let mut scid = [0; quiche::MAX_CONN_ID_LEN];
    // SystemRandom::new().fill(&mut scid[..]).unwrap();
    let scid = quiche::ConnectionId::from_ref(&scid);

    let mut connection = quiche::connect(
        Some("TestServer"),
        &scid,
        socket.local_addr()?,
        server_addr,
        &mut config,
    )?;

    info!(
        "connecting to {:} from {:} with scid {:x?}",
        &server_addr,
        socket.local_addr()?,
        &scid,
    );

    let mut sock_buffer = vec![0; MAX_DATAGRAM_SIZE];
    let (len, send_info) = connection.send(&mut sock_buffer)?;
    socket.send_to(&sock_buffer[..len], &send_info.to).await?;

    let sender = Arc::new(Mutex::new(connection));
    let receiver = sender.clone();
    let stream_id = 0;

    // if connection.is_established() {
    //     // let mut recv_stream = vec![0; MAX_DATAGRAM_SIZE];
    //     // tokio::spawn(async move {
    //     //     loop {
    //     //         let mut receiver = receiver.lock().await;
    //     //         let (len, is_fin) = receiver.stream_recv(stream_id, &mut recv_stream).unwrap();
    //     //         info!("{}, {}", len, is_fin);
    //     //     }
            
    //     // });
    // }
    let mut send_stream = vec![0; MAX_DATAGRAM_SIZE];
    for packet in 0..100 {
        let mut sender = sender.lock().await;
        send_stream.put_u64(packet as u64);
        sender.stream_send(stream_id, &send_stream, true)?;
        sleep(Duration::from_millis(500)).await;
    }
    Ok(())
}
