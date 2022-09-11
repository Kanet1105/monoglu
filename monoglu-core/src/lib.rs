mod storage;

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

pub async fn run_app() {
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();

    let app = Router::new()
        .route("/:file_name", get(get_file));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Listening on {}..", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_file(file_name: String) -> String {
    file_name
}