use crate::error::AppError;
use async_trait::async_trait;
use oauth2::TokenResponse;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenUrl,
};
use reqwest::Client as HttpClient;
use serde::Deserialize;

#[async_trait]
pub trait OAuthService: Send + Sync {
    fn get_authorize_url(&self) -> (String, CsrfToken);
    async fn exchange_code(&self, code: String) -> Result<String, AppError>;
}

pub struct OAuthServiceImpl {
    oauth_client: BasicClient,
    http_client: HttpClient,
}

impl OAuthServiceImpl {
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    ) -> Self {
        let oauth_client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        Self {
            oauth_client,
            http_client: HttpClient::new(),
        }
    }
}

#[async_trait]
impl OAuthService for OAuthServiceImpl {
    fn get_authorize_url(&self) -> (String, CsrfToken) {
        let (auth_url, csrf_token) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read".to_string()))
            .url();
        (auth_url.to_string(), csrf_token)
    }

    async fn exchange_code(&self, code: String) -> Result<String, AppError> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(token.access_token().secret().to_string())
    }
}
