use anyhow::Error;
use monoglu_worker::run_app;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    run_app().await?;
    Ok(())
}
