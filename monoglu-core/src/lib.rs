mod auth;
mod error;

pub use crate::error::*;
use actix_web::{
    get,
    http::{header, StatusCode},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use auth::{AuthClient, AuthClientBuilder, AuthRequest};
use config::Config;
use std::collections::HashMap;
use tracing::{error, info};

/// Read config once and for all at the beginning of building the state.
/// Once the Config is returned, pass the config as a reference to all
/// the other "load_" prefixed functions.
pub fn load_config() -> Result<Config, Exception> {
    let mut config_path = std::env::current_dir()?;
    config_path.push("Config.toml");

    match config_path.exists() {
        true => {
            let config = Config::builder()
                .add_source(config::File::from(config_path.as_ref()))
                .build()?;
            info!("Load configurations at {}", config_path.display());
            Ok(config)
        }
        false => Err(ConfigError::PathError(config_path).into()),
    }
}

pub async fn run_app() -> Result<(), Exception> {
    let config = load_config()?;
    info!("Initialized configurations");

    let client_group = web::Data::new(AuthClientBuilder::from_config(&config)?);
    info!("Successfully loaded auth clients.");

    info!("Initializing the server at http://127.0.0.1:8080..");
    HttpServer::new(move || {
        App::new()
            .app_data(client_group.clone())
            .service(index)
            .service(auth_init)
            .service(auth_ok)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}

#[get("/")]
pub async fn index(
    clients: web::Data<HashMap<String, AuthClient>>,
) -> HttpResponse {
    let mut href = String::new();
    for (name, _) in clients.iter() {
        let a = format!(
            "<div><a href={}>{}</a></div>",
            format!("auth/{}", name),
            name,
        );
        href.push_str(&a);
    }

    let response = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(href);
    response
}

#[get("/auth/{target}")]
pub async fn auth_init(
    client_group: web::Data<HashMap<String, AuthClient>>,
    target_service: web::Path<String>,
) -> Result<HttpResponse, UserError> {
    let target = target_service.into_inner();
    if let Some(auth_client) = client_group.get(&target) {
        let auth_url = auth_client.auth_url();
        let a = format!("<a href={}>link</a>", auth_url);
        let response = HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(a);
        Ok(response)
    } else {
        Err(UserError::InternalError)
    }
}

#[get("/auth/{target}/ok")]
pub async fn auth_ok(auth_query: web::Query<AuthRequest>) -> impl Responder {
    info!("Hello, login!");
    // info!("{:?}", auth_query);
    // let info = user_info.into_inner();
    // let response = HttpResponse::build(StatusCode::OK)
    //     .content_type("text/html; charset=utf-8")
    //     .body("Hello, login!");
    HttpResponse::Ok().finish()
}
