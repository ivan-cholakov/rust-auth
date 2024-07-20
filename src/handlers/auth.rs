use crate::error::AppError;
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::routes::AppState;
use crate::services::{AuthService, OAuthService, SiweService};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = state.auth_service.register(req).await?;
    Ok(Json(res))
}

pub async fn login(
    State(auth_service): State<Arc<dyn AuthService>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let res = auth_service.login(req).await?;
    Ok(Json(res))
}

pub async fn logout() -> &'static str {
    // In a stateless JWT-based auth system, logout is typically handled client-side
    // by removing the token. Here we just return a success message.
    "Logged out successfully"
}

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

pub async fn oauth_login(
    State(oauth_service): State<Arc<dyn OAuthService>>,
) -> Result<String, AppError> {
    let (auth_url, _csrf_token) = oauth_service.get_authorize_url();
    Ok(auth_url)
}

pub async fn oauth_callback(
    State(oauth_service): State<Arc<dyn OAuthService>>,
    Query(params): Query<OAuthCallback>,
) -> Result<Json<AuthResponse>, AppError> {
    let token = oauth_service.exchange_code(params.code).await?;
    Ok(Json(AuthResponse { token }))
}

#[derive(Deserialize)]
pub struct SiweRequest {
    message: String,
    signature: String,
}

pub async fn siwe_login(
    State(siwe_service): State<Arc<dyn SiweService>>,
    Json(req): Json<SiweRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let address = siwe_service
        .verify_signature(req.message, req.signature)
        .await?;
    // Here you would typically create or fetch a user based on the Ethereum address
    // and generate a JWT token. For simplicity, we're just returning the address as the token.
    Ok(Json(AuthResponse {
        token: address.to_string(),
    }))
}
