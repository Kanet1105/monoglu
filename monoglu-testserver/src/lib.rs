mod data;

use actix_files as fs;
use actix_web::{
    get, post, web, 
    App, Error, HttpRequest, HttpServer, HttpResponse
};
use data::*;
use futures::StreamExt;
use std::path::PathBuf;
use tracing::info;

#[get("/")]
async fn index() -> Result<fs::NamedFile, Error> {
    let path = PathBuf::from("C:\\Projects\\monoglu\\monoglu-webapp\\dist\\index.html");
    let file = fs::NamedFile::open(path)?;
    Ok(file)
}

#[get("/{file_name}")]
async fn static_file(request: HttpRequest) -> Result<fs::NamedFile, Error> {
    let mut path = PathBuf::from("C:\\Projects\\monoglu\\monoglu-webapp\\dist");
    let file_name = request.match_info().query("file_name");
    path.push(file_name);
    let file = fs::NamedFile::open(path)?;
    Ok(file)
}

#[post("/auth")]
async fn auth(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let user_info = serde_json::from_slice::<UserInfo>(&body)?;
    info!("{:?}", user_info);
    Ok(HttpResponse::Ok().json(user_info.id))
}

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing the server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(static_file)
            .service(auth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}
