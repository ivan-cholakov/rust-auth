use crate::error::AppError;
use crate::models::User;
use crate::repositories::UserRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_all_users(&self) -> Result<Vec<User>, AppError>;
}

pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.user_repository.get_all_users().await
    }
}
