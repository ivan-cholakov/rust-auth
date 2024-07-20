use crate::error::AppError;
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::models::User;
use crate::repositories::UserRepository;
use async_trait::async_trait;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError>;
    async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError>;
}

pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    jwt_secret: String,
}

impl AuthServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    fn generate_token(&self, user_id: i32) -> Result<String, AppError> {
        #[derive(Serialize, Deserialize)]
        struct Claims {
            sub: i32,
            exp: u64,
        }

        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(60 * 60))
            .expect("Invalid timestamp")
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalServerError)
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        let password_hash = hash(req.password, 10).map_err(|_| AppError::InternalServerError)?;

        let user = self
            .user_repository
            .create_user(&req.username, &req.email, &password_hash)
            .await?;

        let token = self.generate_token(user.id)?;
        Ok(AuthResponse { token })
    }

    async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = self
            .user_repository
            .get_user_by_username(&req.username)
            .await?;

        if verify(&req.password, &user.password_hash).map_err(|_| AppError::InternalServerError)? {
            let token = self.generate_token(user.id)?;
            Ok(AuthResponse { token })
        } else {
            Err(AppError::Unauthorized)
        }
    }
}
