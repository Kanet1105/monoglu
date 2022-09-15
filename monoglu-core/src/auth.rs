use crate::error::Exception;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub(crate) struct ClientBuilder {
    id: String,
    secret: String,
    auth_url: String,
    token_url: String,
    redirect_uri: String,
}

impl ClientBuilder {
    pub fn new(self) -> Result<BasicClient, Exception> {
        let client = BasicClient::new(
            ClientId::new(self.id),
            Some(ClientSecret::new(self.secret)),
            AuthUrl::new(self.auth_url)?,
            Some(TokenUrl::new(self.token_url)?),
        )
        .set_redirect_uri(RedirectUrl::new(self.redirect_uri)?);
        Ok(client)
    }
}
