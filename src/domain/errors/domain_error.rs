use crate::domain::errors::{
    cripto_error::CriptoError, jwt_error::JwtError, repository_error::RepositoryError,
    user_error::UserError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("cryptography error {0}")]
    Cripto(#[from] CriptoError),

    #[error("user error: {0}")]
    User(#[from] UserError),

    #[error("jwt error: {0}")]
    Jwt(#[from] JwtError),
}
