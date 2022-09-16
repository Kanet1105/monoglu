use crate::error::*;
use config::Config;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Clone)]
pub struct AuthClient {
    client: BasicClient,
    scopes: Vec<Scope>,
}

impl AuthClient {
    pub fn new(client: BasicClient, scopes: Vec<Scope>) -> Self {
        Self { client, scopes }
    }

    pub fn auth_url(&self) -> Url {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            // .set_pkce_challenge(pkce_challenge)
            .url();
        auth_url
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
        
        let scopes: Vec<Scope> = self.scopes
            .into_iter()
            .map(|x| Scope::new(x))
            .collect();

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
pub struct AuthRequest {
    code: String,
    state: String,
}
