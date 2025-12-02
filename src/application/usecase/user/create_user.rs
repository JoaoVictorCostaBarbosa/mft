use std::sync::Arc;

use crate::{
    application::dtos::user::{
        auth_reponse::AuthResponse, user_create::UserCreate, user_response::UserResponse,
    },
    domain::{
        entities::user::User,
        errors::{domain_error::DomainError, repository_error::RepositoryError, user_error::UserError},
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, jwt::JwtProvider},
        value_objects::password_vo::Password,
    },
};

#[derive(Clone)]
pub struct CreateUser {
    pub user_repository: Arc<dyn UserRepository>,
    pub cripto_service: Arc<dyn CriptoService>,
    pub jwt_service: Arc<dyn JwtProvider>,
}

impl CreateUser {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        cripto_service: Arc<dyn CriptoService>,
        jwt_service: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            user_repository,
            cripto_service,
            jwt_service,
        }
    }

    pub async fn execute(&self, user_data: UserCreate) -> Result<AuthResponse, DomainError> {
        let existing_user: Option<User> = self.user_repository
            .get_user_by_email(user_data.email.as_str())
            .await
            .ok();
        
        if let Some(_) = existing_user {
            return Err(DomainError::Repository(RepositoryError::Conflict("email already used".to_string())));
        }
        
        let password = Password::new(user_data.password).map_err(UserError::from)?;
        let password_hash = self.cripto_service.hash(password.value())?;

        let user = User::new(user_data.name, user_data.email, password_hash)?;

        (self.user_repository.create_user(&user)).await?;

        let access = self
            .jwt_service
            .generate_access(user.id.to_string(), user.role)?;
        let refresh = self.jwt_service.generate_refresh(user.id.to_string())?;

        let response = AuthResponse::new(UserResponse::to_reponse(user), access, refresh);

        Ok(response)
    }
}
