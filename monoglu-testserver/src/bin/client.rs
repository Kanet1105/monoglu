use monoglu_testserver::prelude::*;
use tracing_subscriber::fmt::time;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_timer(time::LocalTime::rfc_3339())
        .init();
    monoglu_testserver::run_client().unwrap();
    info!("Shutting down..");
    Ok(())
}
