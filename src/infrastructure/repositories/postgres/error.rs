use crate::domain::errors::{domain_error::DomainError, repository_error::RepositoryError};

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DomainError::Repository(RepositoryError::NotFound("".to_string())),
            sqlx::Error::Database(err) => {
                match err.code().as_deref() {
                    Some("23505") => DomainError::Repository(RepositoryError::Conflict(err.message().to_string())),
                    _ => DomainError::Repository(RepositoryError::DbError(err.message().to_string())),
                }
            }
            
            _ => DomainError::Repository(RepositoryError::Unexpected("Unexpected error".to_string()))
        }
    }
}
