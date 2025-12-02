use crate::{
    application::dtos::user::{
        auth_reponse::AuthResponse, login_request::LoginRequest, user_response::UserResponse,
    },
    domain::{
        entities::user::User,
        errors::{cripto_error::CriptoError, domain_error::DomainError, user_error::UserError},
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, jwt::JwtProvider},
        value_objects::email_vo::Email,
    },
};
use std::sync::Arc;

pub struct LoginUser {
    pub user_repo: Arc<dyn UserRepository>,
    pub cripto_service: Arc<dyn CriptoService>,
    pub jwt_service: Arc<dyn JwtProvider>,
}

impl LoginUser {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        cripto_service: Arc<dyn CriptoService>,
        jwt_service: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            user_repo,
            cripto_service,
            jwt_service,
        }
    }

    pub async fn execute(&self, user_data: LoginRequest) -> Result<AuthResponse, DomainError> {
        let _email = Email::new(user_data.email.clone())
            .map_err(|e| DomainError::User(UserError::EmailInvalid(e)))?;

        let user: User = self
            .user_repo
            .get_user_by_email(user_data.email.as_str())
            .await
            .map_err(|_| DomainError::Cripto(CriptoError::InvalidCredentials))?;

        if !(self
            .cripto_service
            .verify(&user_data.password, &user.password))?
        {
            return Err(DomainError::Cripto(CriptoError::InvalidCredentials));
        }

        let access = self
            .jwt_service
            .generate_access(user.id.to_string(), user.role)?;
        let refresh = self.jwt_service.generate_refresh(user.id.to_string())?;

        let response = AuthResponse {
            user: UserResponse::to_reponse(user),
            access,
            refresh,
        };

        Ok(response)
    }
}
