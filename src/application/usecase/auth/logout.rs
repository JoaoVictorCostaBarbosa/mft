use crate::domain::{
    errors::{domain_error::DomainError, permission_error::PermissionError},
    repositories::refresh_token_repository::RefreshTokenRepository,
    services::refresh_token_hasher::RefreshTokenHasher,
};
use std::sync::Arc;

pub struct Logout {
    pub refresh_repo: Arc<dyn RefreshTokenRepository>,
    pub hash_service: Arc<dyn RefreshTokenHasher>,
}

impl Logout {
    pub fn new(
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hash_service: Arc<dyn RefreshTokenHasher>,
    ) -> Self {
        Self {
            refresh_repo,
            hash_service,
        }
    }

    pub async fn execute(&self, token: String) -> Result<(), DomainError> {
        let hashed_token = self.hash_service.hash(&token)?;

        let refresh_token = match self.refresh_repo.find_valid_by_hash(&hashed_token).await {
            Ok(t) => t,
            Err(_) => return Err(PermissionError::Unauthorized.into()),
        };

        self.refresh_repo.revoke(refresh_token.id).await?;

        Ok(())
    }
}
