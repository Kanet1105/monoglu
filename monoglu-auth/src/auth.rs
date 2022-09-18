use crate::error::*;
use config::Config;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    mem::replace,
    ops::Deref,
    sync::Mutex,
};
use tracing::info;
use url::Url;

pub struct AuthCache(Mutex<HashMap<String, PkceCodeVerifier>>);

impl AuthCache {
    pub fn new() -> Self {
        let inner = HashMap::<String, PkceCodeVerifier>::new();
        Self(Mutex::new(inner))
    }

    pub fn new_csrf_token(&self, csrf_token: CsrfToken, pkce_verifier: PkceCodeVerifier) {
        let mut cache_guard = self.lock().unwrap();

        let csrf_string: String = csrf_token.secret().into();
        cache_guard.insert(csrf_string, pkce_verifier);
    }

    pub fn find_pkce_verifier(&self, csrf_token: &str) -> Result<PkceCodeVerifier, UserError> {
        let mut cache_guard = self.lock().unwrap();

        match cache_guard.get_mut(csrf_token) {
            Some(pkce_verifier) => {
                let verifier = replace(pkce_verifier, PkceCodeVerifier::new("".to_string()));
                Ok(verifier)
            },
            None => Err(UserError::AuthClientNotFound),
        }
    }
}

impl Deref for AuthCache {
    type Target = Mutex<HashMap<String, PkceCodeVerifier>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct AuthClient {
    client: BasicClient,
    scopes: Vec<Scope>,
}

impl AuthClient {
    pub fn new(client: BasicClient, scopes: Vec<Scope>) -> Self {
        Self { client, scopes }
    }

    pub fn auth_url(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .set_pkce_challenge(pkce_challenge)
            .url();
        (auth_url, csrf_token, pkce_verifier)
    }

    pub async fn token_url(
        &self,
        auth_code: String,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<(), Exception> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(auth_code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await?;
        info!("{:?}", token.access_token().secret());
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthClientBuilder {
    id: String,
    secret: String,
    auth_url: String,
    token_url: String,
    redirect_uri: String,
    scopes: Vec<String>,
}

impl AuthClientBuilder {
    fn build(self) -> Result<AuthClient, Exception> {
        let client = BasicClient::new(
            ClientId::new(self.id),
            Some(ClientSecret::new(self.secret)),
            AuthUrl::new(self.auth_url)?,
            Some(TokenUrl::new(self.token_url)?),
        )
        .set_redirect_uri(RedirectUrl::new(self.redirect_uri)?);

        let scopes: Vec<Scope> = self.scopes.into_iter().map(|x| Scope::new(x)).collect();

        let auth_client = AuthClient::new(client, scopes);
        Ok(auth_client)
    }

    pub fn from_config(config: &Config) -> Result<HashMap<String, AuthClient>, Exception> {
        let table = "oauth2".to_string();
        let oauth2 = config.get_table(&table)?;
        if oauth2.is_empty() {
            return Err(ConfigError::EmptyTableError(table).into());
        }

        let mut client_group = HashMap::<String, AuthClient>::new();
        for (name, value) in oauth2 {
            let client_builder = value.try_deserialize::<AuthClientBuilder>()?;
            let auth_client = client_builder.build()?;
            client_group.insert(name, auth_client);
        }
        Ok(client_group)
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub state: String,
    pub code: String,
}
