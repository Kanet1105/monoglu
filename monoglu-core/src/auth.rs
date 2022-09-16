use crate::error::*;
use config::Config;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use url::Url;

#[derive(Clone)]
pub struct AuthClient(HashMap<String, BasicClient>);

impl AuthClient {
    pub fn new() -> Self {
        Self(HashMap::<String, BasicClient>::new())
    }
}

impl Deref for AuthClient {
    type Target = HashMap<String, BasicClient>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AuthClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthClientBuilder {
    id: String,
    secret: String,
    auth_url: String,
    token_url: String,
    redirect_uri: String,
}

impl AuthClientBuilder {
    fn new(self) -> Result<BasicClient, Exception> {
        let client = BasicClient::new(
            ClientId::new(self.id),
            Some(ClientSecret::new(self.secret)),
            AuthUrl::new(self.auth_url)?,
            Some(TokenUrl::new(self.token_url)?),
        )
        .set_redirect_uri(RedirectUrl::new(self.redirect_uri)?);
        Ok(client)
    }

    pub fn from_config(config: &Config) -> Result<AuthClient, Exception> {
        let table = "oauth2".to_string();
        let oauth2 = config.get_table(&table)?;
        if oauth2.is_empty() {
            return Err(ConfigError::EmptyTableError(table).into());
        }

        let mut auth_client = AuthClient::new();
        for (name, value) in oauth2 {
            let client = value.try_deserialize::<AuthClientBuilder>()?.new()?;
            auth_client.insert(name, client);
        }
        Ok(auth_client)
    }
}
