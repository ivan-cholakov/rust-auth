use crate::error::AppError;
use async_trait::async_trait;
use ethers::types::{Address, Signature};
use siwe::{Message, VerificationOpts};
use std::{str::FromStr, time::Duration};
use time::OffsetDateTime;

#[async_trait]
pub trait SiweService: Send + Sync {
    async fn verify_signature(
        &self,
        message: String,
        signature: String,
    ) -> Result<Address, AppError>;
}

pub struct SiweServiceImpl;

impl SiweServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SiweService for SiweServiceImpl {
    async fn verify_signature(
        &self,
        message: String,
        signature: String,
    ) -> Result<Address, AppError> {
        let message = Message::from_str(&message)
            .map_err(|_| AppError::BadRequest("Invalid message".to_string()))?;
        let signature = Signature::from_str(&signature)
            .map_err(|_| AppError::BadRequest("Invalid signature".to_string()))?;

        message
            .verify(
                signature.to_vec().as_slice(),
                &VerificationOpts {
                    domain: None,
                    nonce: None,
                    timestamp: Some(OffsetDateTime::now_utc()),
                },
            )
            .await
            .map_err(|_| AppError::Unauthorized)?;

        Ok(ethers::types::H160(message.address))
    }
}
