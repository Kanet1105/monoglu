use crate::prelude::*;
use std::fs;
use std::path::PathBuf;

#[allow(unused)]
pub fn generate_pem(path: &str, name: &str) -> Result<()> {
    fs::create_dir_all(path)?;
    let mut cert_path = PathBuf::from(path);
    cert_path.push(format!("{}_cert.pem", name));
    let mut key_path = PathBuf::from(path);
    key_path.push(format!("{}_key.pem", name));

    let cert = rcgen::generate_simple_self_signed(vec![format!("{}", name)])?;
    fs::write(&cert_path, cert.serialize_pem()?)?;
    fs::write(&key_path, cert.serialize_private_key_pem())?;
    Ok(())
}
