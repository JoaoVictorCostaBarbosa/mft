use crate::{
    application::{
        interfaces::pending_user_repository::PendingUserRepository,
        usecase::user::{create_user::CreateUser, login_user::LoginUser, verify_user::VerifyUser},
    },
    domain::{
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, jwt::JwtProvider, smtp::SmtpService},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthAppState {
    pub create_user: Arc<CreateUser>,
    pub verify_user: Arc<VerifyUser>,
    pub login_user: Arc<LoginUser>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl AuthAppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        jwt_service: Arc<dyn JwtProvider>,
        cripto_service: Arc<dyn CriptoService>,
        smtp_service: Arc<dyn SmtpService>,
    ) -> Self {
        Self {
            create_user: Arc::new(CreateUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
            )),
            verify_user: Arc::new(VerifyUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                jwt_service.clone(),
            )),
            login_user: Arc::new(LoginUser::new(
                user_repo.clone(),
                cripto_service.clone(),
                jwt_service.clone(),
            )),
            user_repo,
        }
    }
}
