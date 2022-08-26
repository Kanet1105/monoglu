mod auth;

pub mod prelude {
    pub use std::collections::BTreeMap;
    pub use std::path::PathBuf;

    pub use actix_files::{Files, NamedFile};
    pub use actix_web::{get, post, web};
    pub use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
    pub use actix_web::middleware::Logger;

    pub use serde::{Serialize, Deserialize};
}

use crate::prelude::*;

#[derive(Serialize)]
pub struct NodeInfo {
    pub user: String,
    pub file_map: BTreeMap<String, Node>,
}

#[derive(Serialize)]
pub struct Node {
    pub node_type: NodeType,
    pub name: String,
    pub parent: Option<Node>,
    pub children: Option<Vec<Node>>,
    pub last_modified: String,
    pub check_sum: String,
}

#[derive(Serialize)]
pub enum NodeType {
    Root,
    Folder,
    File,
}

#[get("/testget")]
pub async fn test_get(request: HttpRequest) -> Result<impl Responder> {
    
    Ok(web::Json(object))
}

/// The entry point of the server.
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        let webapp = Files::new("/", "D:\\RustProjects\\monoglu\\monoglu-webapp\\dist")
            .index_file("D:\\RustProjects\\monoglu\\monoglu-webapp\\dist\\index.html");

        App::new()
            .wrap(Logger::default())
            .service(test_get)
            .service(webapp)
    })
    .bind("127.0.0.1:60000")?
    .run()
    .await
}