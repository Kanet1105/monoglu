mod config;

pub mod prelude {
    pub type Exception = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Exception>;

    pub use crate::config::Configuration;
    pub use tracing::{error, info};
}

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

use crate::prelude::*;
use std::sync::Arc;

#[tokio::main]
pub async fn run_server() -> Result<()> {
    let config = Configuration::from_file("./Config.toml")?;
    let (cert, key) = config.tls.load_cert()?;
    
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;
    server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(server_crypto));
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_concurrent_uni_streams(config.server.max_concurrency.into());

    let endpoint = quinn::Endpoint::server(server_config, config.server.endpoint_v4()?)?;
    info!("Listening on {}", endpoint.local_addr()?);

    while let Some(connection) = endpoint.accept().await {
        info!("{:?}", connection);
    }

    Ok(())
}
