use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::{self, auth};
use crate::services::{AuthService, OAuthService, SiweService, UserService};
use tower_http::services::ServeDir;

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
        .route("/", get(handlers::user::get_users))
        .route("/users", get(handlers::user::get_users))
        .route("/register", get(auth::show_register).post(auth::register))
        .route("/login", get(auth::show_login).post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/oauth/login", get(auth::oauth_login))
        .route("/oauth/callback", get(auth::oauth_callback))
        .route("/siwe/login", post(auth::siwe_login))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
