#[actix_web::main]
async fn main() -> std::io::Result<()> {
    monoglu_server::run().await?;
    Ok(())
}