use crate::{
    application::dtos::auth::login_request::LoginRequest,
    domain::{
        entities::user::User,
        errors::{
            domain_error::DomainError, permission_error::PermissionError, user_error::UserError,
        },
        repositories::user_repository::UserRepository,
        services::cripto::CriptoService,
        value_objects::email_vo::Email,
    },
};
use std::sync::Arc;

pub struct LoginUser {
    pub user_repo: Arc<dyn UserRepository>,
    pub cripto_service: Arc<dyn CriptoService>,
}

impl LoginUser {
    pub fn new(user_repo: Arc<dyn UserRepository>, cripto_service: Arc<dyn CriptoService>) -> Self {
        Self {
            user_repo,
            cripto_service,
        }
    }

    pub async fn execute(&self, user_data: LoginRequest) -> Result<User, DomainError> {
        let _email = Email::new(user_data.email.clone())
            .map_err(|e| DomainError::User(UserError::EmailInvalid(e)))?;

        let user: User = self
            .user_repo
            .get_user_by_email(user_data.email.as_str())
            .await
            .map_err(|_| DomainError::Permisson(PermissionError::Unauthorized))?;

        if user.deleted_at.is_some() {
            return Err(DomainError::Permisson(PermissionError::Unauthorized));
        }

        if !(self
            .cripto_service
            .verify(&user_data.password, &user.password))?
        {
            return Err(DomainError::Permisson(PermissionError::Unauthorized));
        }

        Ok(user)
    }
}
