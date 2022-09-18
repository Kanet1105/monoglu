mod auth;
mod error;

pub use crate::error::*;
use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use auth::{AuthCache, AuthClient, AuthClientBuilder, AuthQuery};
use config::Config;
use std::collections::HashMap;
use tracing::info;

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

    let auth_cache = web::Data::new(AuthCache::new());
    info!("Initialized Oauth 2 request cache.");

    let client_group = web::Data::new(AuthClientBuilder::from_config(&config)?);
    info!("Initialized Oauth 2 clients.");

    info!("Initializing the server at http://127.0.0.1:8080..");
    HttpServer::new(move || {
        App::new()
            .app_data(auth_cache.clone())
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
pub async fn index(clients: web::Data<HashMap<String, AuthClient>>) -> HttpResponse {
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
    auth_cache: web::Data<AuthCache>,
    auth_client: web::Data<HashMap<String, AuthClient>>,
    target_service: web::Path<String>,
) -> Result<HttpResponse, UserError> {
    let target = target_service.into_inner();

    if let Some(client) = auth_client.get(&target) {
        let (auth_url, csrf_token, pkce_verifier) = client.auth_url();
        auth_cache.new_csrf_token(csrf_token, pkce_verifier);

        let a = format!("<a href={}>link</a>", auth_url);
        let response = HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(a);
        info!("Generated a new session for a login request.");
        Ok(response)
    } else {
        Err(UserError::InternalError)
    }
}

#[get("/auth/{target}/ok")]
pub async fn auth_ok(
    auth_cache: web::Data<AuthCache>,
    auth_client: web::Data<HashMap<String, AuthClient>>,
    auth_query: web::Query<AuthQuery>,
    target_service: web::Path<String>,
) -> Result<HttpResponse, UserError> {
    let target = target_service.into_inner();
    
    if let Some(client) = auth_client.get(&target) {
        let pkce_verifier = auth_cache.find_pkce_verifier(&auth_query.state)?;
        let auth_code: String = (&*auth_query.code).into();

        client.token_url(auth_code, pkce_verifier).await.unwrap();
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(UserError::InternalError)
    }
}
