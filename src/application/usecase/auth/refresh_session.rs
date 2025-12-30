use crate::{
    application::dtos::auth::refresh_response::RefreshResponse,
    domain::{
        entities::refresh_token::RefreshToken,
        errors::{
            domain_error::DomainError, permission_error::PermissionError,
            repository_error::RepositoryError,
        },
        repositories::{
            refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
        },
        services::{jwt::JwtProvider, refresh_token_hasher::RefreshTokenHasher},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RefreshSession {
    pub refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub jwt_service: Arc<dyn JwtProvider>,
    pub hash_service: Arc<dyn RefreshTokenHasher>,
    pub refresh_exp_in_days: i64,
}

impl RefreshSession {
    pub fn new(
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        user_repo: Arc<dyn UserRepository>,
        jwt_service: Arc<dyn JwtProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        refresh_exp_in_days: i64,
    ) -> Self {
        Self {
            refresh_token_repo,
            user_repo,
            jwt_service,
            hash_service,
            refresh_exp_in_days,
        }
    }

    pub async fn execute(&self, token: String) -> Result<RefreshResponse, DomainError> {
        let hashed_token = self.hash_service.hash(&token)?;

        let refresh_token = match self
            .refresh_token_repo
            .find_valid_by_hash(&hashed_token)
            .await
        {
            Ok(t) => t,
            Err(RepositoryError::NotFound(_)) => return Err(PermissionError::Unauthorized.into()),
            Err(e) => return Err(e.into()),
        };

        self.refresh_token_repo.revoke(refresh_token.id).await?;

        let user = self.user_repo.get_user_by_id(refresh_token.user_id).await?;

        let access = self
            .jwt_service
            .generate_access(user.id.to_string(), user.role)?;

        let refresh_raw = Uuid::new_v4().to_string();
        let refresh_hash = self.hash_service.hash(&refresh_raw)?;

        let refresh = RefreshToken::new(user.id, refresh_hash, self.refresh_exp_in_days);

        self.refresh_token_repo.create(refresh).await?;

        let response = RefreshResponse::new(access, refresh_raw);

        Ok(response)
    }
}
