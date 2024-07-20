use crate::error::AppError;
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::routes::api_v1::AppState;
use crate::services::{AuthService, OAuthService, SiweService};
use crate::templates::{LoginTemplate, RegisterTemplate};
use askama_axum::IntoResponse;
use askama_axum::Template;
use axum::{extract::Query, Json};
use axum::{extract::State, response::Html, Form};
use serde::Deserialize;
use std::sync::Arc;

pub async fn show_register() -> impl IntoResponse {
    let template = RegisterTemplate {};
    Html(template.render().unwrap())
}

pub async fn show_login() -> impl IntoResponse {
    let template = LoginTemplate {};
    Html(template.render().unwrap())
}

pub async fn register(
    State(state): State<AppState>,
    Form(req): Form<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res = state.auth_service.register(req).await?;
    Ok(format!("Registered successfully! Token: {}", res.token))
}

pub async fn login(
    State(state): State<AppState>,
    Form(req): Form<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res = state.auth_service.login(req).await?;
    Ok(format!("Logged in successfully! Token: {}", res.token))
}

pub async fn logout() -> impl IntoResponse {
    "Logged out successfully"
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

pub async fn oauth_login(State(state): State<AppState>) -> Result<String, AppError> {
    let (auth_url, _csrf_token) = state.oauth_service.get_authorize_url();
    Ok(auth_url)
}

pub async fn oauth_callback(
    State(state): State<AppState>,
    Query(params): Query<OAuthCallback>,
) -> Result<Json<AuthResponse>, AppError> {
    let token = state.oauth_service.exchange_code(params.code).await?;
    Ok(Json(AuthResponse { token }))
}

#[derive(Deserialize)]
pub struct SiweRequest {
    message: String,
    signature: String,
}

pub async fn siwe_login(
    State(state): State<AppState>,
    Json(req): Json<SiweRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let address = state
        .siwe_service
        .verify_signature(req.message, req.signature)
        .await?;
    // Here you would typically create or fetch a user based on the Ethereum address
    // and generate a JWT token. For simplicity, we're just returning the address as the token.
    Ok(Json(AuthResponse {
        token: address.to_string(),
    }))
}
