use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::{self, auth};
use crate::services::{AuthService, OAuthService, SiweService, UserService};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub auth_service: Arc<dyn AuthService>,
    pub oauth_service: Arc<dyn OAuthService>,
    pub siwe_service: Arc<dyn SiweService>,
}

pub fn create_router(
    user_service: Arc<dyn UserService>,
    auth_service: Arc<dyn AuthService>,
    oauth_service: Arc<dyn OAuthService>,
    siwe_service: Arc<dyn SiweService>,
) -> Router {
    let state = AppState {
        user_service,
        auth_service,
        oauth_service,
        siwe_service,
    };

    Router::new()
        .route("/users", get(handlers::user::get_users))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/oauth/login", get(auth::oauth_login))
        .route("/oauth/callback", get(auth::oauth_callback))
        .route("/siwe/login", post(auth::siwe_login))
        .with_state(state)
}
