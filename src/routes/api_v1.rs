use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::{
    handlers::product,
    services::{AuthService, OAuthService, SiweService, UserService},
};
use crate::{
    handlers::{self, auth},
    services::ProductService,
};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub auth_service: Arc<dyn AuthService>,
    pub oauth_service: Arc<dyn OAuthService>,
    pub siwe_service: Arc<dyn SiweService>,
    pub product_service: Arc<dyn ProductService>,
}

pub fn create_router(
    user_service: Arc<dyn UserService>,
    auth_service: Arc<dyn AuthService>,
    oauth_service: Arc<dyn OAuthService>,
    siwe_service: Arc<dyn SiweService>,
    product_service: Arc<dyn ProductService>,
) -> Router {
    let state = AppState {
        user_service,
        auth_service,
        oauth_service,
        siwe_service,
        product_service,
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
        .route("/products", get(product::get_products).post(product::create_product))
        .route("/products/new", get(product::new_product))
        .route("/products/:id", get(product::get_product).put(product::update_product).delete(product::delete_product))
        .route("/products/:id/edit", get(product::edit_product))
        .route("/bundles", get(product::get_bundles).post(product::create_bundle))
        .route("/bundles/new", get(product::new_bundle))
        .route("/bundles/:id", get(product::get_bundle).put(product::update_bundle).delete(product::delete_bundle))
        .route("/bundles/:id/edit", get(product::edit_bundle))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
