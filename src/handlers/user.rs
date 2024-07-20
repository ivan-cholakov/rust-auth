use crate::error::AppError;
use crate::models::User;
use crate::services::UserService;
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn get_users(
    State(user_service): State<Arc<dyn UserService>>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = user_service.get_all_users().await?;
    Ok(Json(users))
}
