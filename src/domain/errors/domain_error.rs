use crate::domain::errors::{
    bucket_error::BucketError, cripto_error::CriptoError,
    exercise_error::ExerciseError, file_error::FileError, jwt_error::JwtError,
    measurement_error::MeasurementError, permission_error::PermissionError,
    repository_error::RepositoryError, smtp_error::SmtpError, user_error::UserError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("cryptography error: {0}")]
    Cripto(#[from] CriptoError),

    #[error("user error: {0}")]
    User(#[from] UserError),

    #[error("jwt error: {0}")]
    Jwt(#[from] JwtError),

    #[error("permisson error: {0}")]
    Permisson(#[from] PermissionError),

    #[error("smtp error: {0}")]
    Smtp(#[from] SmtpError),

    #[error("bucker error: {0}")]
    Bucket(#[from] BucketError),

    #[error("file error: {0}")]
    File(#[from] FileError),

    #[error("measurement error: {0}")]
    Measurement(#[from] MeasurementError),

    #[error("exercise error: {0}")]
    Exercise(#[from] ExerciseError),
}
