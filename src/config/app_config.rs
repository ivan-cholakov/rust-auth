use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
    pub jwt_secret: String,
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub oauth_auth_url: String,
    pub oauth_token_url: String,
    pub oauth_redirect_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(AppConfig {
            database_url: env::var("DATABASE_URL")?,
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            jwt_secret: env::var("JWT_SECRET")?,
            oauth_client_id: env::var("OAUTH_CLIENT_ID")?,
            oauth_client_secret: env::var("OAUTH_CLIENT_SECRET")?,
            oauth_auth_url: env::var("OAUTH_AUTH_URL")?,
            oauth_token_url: env::var("OAUTH_TOKEN_URL")?,
            oauth_redirect_url: env::var("OAUTH_REDIRECT_URL")?,
        })
    }
}
