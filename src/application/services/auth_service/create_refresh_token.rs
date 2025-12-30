use crate::domain::{
    entities::refresh_token::RefreshToken, errors::domain_error::DomainError,
    repositories::refresh_token_repository::RefreshTokenRepository,
    services::refresh_token_hasher::RefreshTokenHasher,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct IssueRefreshToken {
    pub refresh_repo: Arc<dyn RefreshTokenRepository>,
    pub hash_service: Arc<dyn RefreshTokenHasher>,
    pub refresh_exp_in_days: i64,
}

impl IssueRefreshToken {
    pub fn new(
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        refresh_exp_in_days: i64,
    ) -> Self {
        Self {
            refresh_repo,
            hash_service,
            refresh_exp_in_days,
        }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<String, DomainError> {
        let refresh_raw = Uuid::new_v4().to_string();
        let refresh_hash = self.hash_service.hash(&refresh_raw)?;

        let refresh = RefreshToken::new(user_id, refresh_hash, self.refresh_exp_in_days);

        self.refresh_repo.create(refresh).await?;

        Ok(refresh_raw)
    }
}
