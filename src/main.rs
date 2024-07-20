use std::sync::Arc;

use sqlx::PgPool;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod templates;

use crate::config::AppConfig;
use crate::db::create_pool;
use crate::repositories::UserRepositoryImpl;
use crate::routes::create_router;
use crate::services::{AuthServiceImpl, OAuthServiceImpl, SiweServiceImpl, UserServiceImpl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env()?;
    let pool: PgPool = create_pool(&config.database_url).await?;
    let pool_arc = Arc::new(pool);

    let user_repository = Arc::new(UserRepositoryImpl::new(pool_arc.clone()));
    let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(
        user_repository.clone(),
        config.jwt_secret.clone(),
    ));
    let oauth_service = Arc::new(OAuthServiceImpl::new(
        config.oauth_client_id,
        config.oauth_client_secret,
        config.oauth_auth_url,
        config.oauth_token_url,
        config.oauth_redirect_url,
    ));
    let siwe_service = Arc::new(SiweServiceImpl::new());

    let app = create_router(user_service, auth_service, oauth_service, siwe_service);

    let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
    println!("Listening on {}", config.server_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
