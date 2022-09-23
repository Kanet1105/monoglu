mod config;
mod errors;

pub mod prelude {
    pub use crate::errors::*;
    pub use anyhow::{Context, Error};
    pub use serde::{Deserialize, Serialize};
    pub use std::{
        env::current_dir,
        fmt::{Debug, Display},
        fs::{self, File},
        io::{Read, Write},
        path::PathBuf,
        sync::{Arc, RwLock},
    };
    pub use tracing::{debug, info};
}

use crate::prelude::*;

pub async fn run_app() -> Result<(), Error> {
    let config = config::ConfigBuilder::new()?;
    Ok(())
}
