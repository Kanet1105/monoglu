use crate::prelude::*;
use rustls::{Certificate, PrivateKey};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub struct TLS {
    cert_path: PathBuf,
    key_path: PathBuf,
}

impl TLS {
    /// Generate a simple & self-signed certificate and
    /// its private key, saving them under the "/cert"
    /// inside the current working directory.
    pub fn generate() -> Result<Self> {
        // Set the certificate directory.
        let mut cert_dir = std::env::current_dir()?;
        cert_dir.push("cert");
        if !cert_dir.exists() {
            fs::create_dir(&cert_dir)?;
        }
        let cert_path = cert_dir.join("cert.der");
        let key_path = cert_dir.join("key.der");

        // Generate a key pair and save them to files.
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        fs::write(&cert_path, cert.serialize_der()?)?;
        fs::write(&key_path, cert.serialize_private_key_der())?;
        Ok(Self { cert_path, key_path })
    }

    /// * [rustls::key::Certificate] => contains public key. \
    /// * [rustls::key::PrivateKey] => The private key must be 
    /// DER-encoded ASN.1 in either PKCS#8 or PKCS#1 format.
    pub fn load_cert(&self) -> Result<(Certificate, PrivateKey)> {
        let cert = Certificate(fs::read(&self.cert_path)?);
        let key = PrivateKey(fs::read(&self.key_path)?);
        Ok((cert, key))
    }
}

impl Default for TLS {
    fn default() -> Self {
        Self::generate().unwrap()
    }
}

#[test]
fn test_load_cert() {
    let cert = TLS::generate().unwrap();
    let (cert, key) = cert.load_cert().unwrap();
    println!("{:?}\n{:?}", cert, key);  
}
