use crate::models::User;
use crate::services::UserService;
use crate::{error::AppError, routes::AppState};
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.user_service.get_all_users().await?;
    Ok(Json(users))
}
