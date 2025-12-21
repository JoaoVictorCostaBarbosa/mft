use crate::{
    adapters::http::errors::http_error::HttpError,
    application::app_state::app_state::AppState,
    domain::{auth::token_data::AccessTokenData, errors::domain_error::DomainError},
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

pub struct AuthClaims(pub AccessTokenData);

#[async_trait]
impl FromRequestParts<AppState> for AuthClaims {
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(HttpError(DomainError::Jwt(
                crate::domain::errors::jwt_error::JwtError::MissingClaim,
            )))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(HttpError(DomainError::Jwt(
                crate::domain::errors::jwt_error::JwtError::MissingClaim,
            )));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        let claims = state
            .jwt_service
            .verify_access(token)
            .map_err(|e| HttpError(DomainError::Jwt(e)))?;

        Ok(AuthClaims(claims))
    }
}
