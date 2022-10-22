#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
    monoglu_testserver::run_app().await?;
    Ok(())
}
