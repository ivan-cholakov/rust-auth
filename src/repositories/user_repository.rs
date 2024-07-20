use crate::error::AppError;
use crate::models::user::User;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, AppError>;
    async fn get_all_users(&self) -> Result<Vec<User>, AppError>;
}

pub struct UserRepositoryImpl {
    pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email, password_hash",
            username,
            email,
            password_hash
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as!(User, "SELECT id, username, email, password_hash FROM users")
            .fetch_all(&*self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(users)
    }
}
