use crate::domain::errors::{
    cripto_error::CriptoError, domain_error::DomainError, repository_error::RepositoryError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        match self {
            DomainError::Repository(RepositoryError::NotFound(msg)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": msg }))).into_response()
            }
            DomainError::Repository(RepositoryError::Conflict(msg)) => {
                (StatusCode::CONFLICT, Json(json!({ "error": msg }))).into_response()
            }
            DomainError::Repository(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            DomainError::Cripto(CriptoError::InvalidCredentials) => {
                (StatusCode::UNAUTHORIZED).into_response()
            }
            DomainError::Cripto(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            DomainError::User(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
            DomainError::Jwt(err) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": err.to_string() })),
            )
                .into_response(),
        }
    }
}
