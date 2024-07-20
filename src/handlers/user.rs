use crate::error::AppError;
use crate::routes::api_v1::AppState;
use crate::templates::UsersTemplate;
use askama_axum::IntoResponse;
use askama_axum::Template;
use axum::extract::State;
use axum::response::Html;

pub async fn get_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let users = state.user_service.get_all_users().await?;
    let template = UsersTemplate { users };
    Ok(Html(template.render().unwrap()))
}
