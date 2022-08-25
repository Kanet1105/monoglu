mod auth;

pub mod prelude {
    pub use std::path::PathBuf;

    pub use actix_files::{Files, NamedFile};
    pub use actix_web::{get, post, web};
    pub use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
    pub use actix_web::middleware::Logger;
}

use crate::prelude::*;

/// The entry point of the server.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        let webapp = Files::new("/", "D:\\RustProjects\\monoglu\\monoglu-webapp\\dist")
            .index_file("D:\\RustProjects\\monoglu\\monoglu-webapp\\dist\\index.html");

        App::new()
            .wrap(Logger::default())
            .service(webapp)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}