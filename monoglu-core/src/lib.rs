mod auth;
mod config;
mod error;
mod state;
mod storage;

use crate::error::Exception;
use axum::{
    // body::Bytes,
    // error_handling::HandleErrorLayer,
    handler::Handler,
    // http::StatusCode,
    // response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use state::SharedState;
// use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};
use tracing::{error, info};
use tracing_subscriber;

pub async fn run_app() {
    // env::set_var("RUST_BACKTRACE", "full");

    // Initialize the global tracing subscriber.
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .init();

    let shared_state = SharedState::new().map_err(|error| {
        error!(error);
        panic!();
    });

    // let database = Storage::new().await.unwrap();
    // info!("Connected to the database.");
    let app = Router::new().route("/", get(google_signin));
    info!("Initialize the server.");

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on {}..", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn google_signin() {}
