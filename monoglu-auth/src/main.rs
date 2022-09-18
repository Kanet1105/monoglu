use monoglu_core::run_app;
use monoglu_core::Exception;

#[actix_web::main]
async fn main() -> Result<(), Exception> {
    // env::set_var("RUST_BACKTRACE", "full");
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .init();
    run_app().await?;
    Ok(())
}
