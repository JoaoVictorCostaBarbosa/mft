use crate::domain::{
    entities::refresh_token::RefreshToken, errors::repository_error::RepositoryError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: RefreshToken) -> Result<(), RepositoryError>;
    async fn find_valid_by_hash(&self, hash: &str) -> Result<RefreshToken, RepositoryError>;
    async fn revoke(&self, token_id: Uuid) -> Result<(), RepositoryError>;
}
