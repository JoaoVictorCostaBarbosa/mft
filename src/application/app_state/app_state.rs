use crate::{
    application::{
        app_state::{auth_app_state::AuthAppState, user_app_state::UserAppState},
        interfaces::{
            pending_change_repository::PendingChangesRepository,
            pending_user_repository::PendingUserRepository,
        },
    },
    domain::{
        repositories::user_repository::UserRepository,
        services::{
            bucket_storage::BucketStorage, cripto::CriptoService, jwt::JwtProvider,
            smtp::SmtpService,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthAppState,
    pub user: UserAppState,
    pub jwt_service: Arc<dyn JwtProvider>,
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        cripto_service: Arc<dyn CriptoService>,
        jwt_service: Arc<dyn JwtProvider>,
        smtp_service: Arc<dyn SmtpService>,
        bucket_service: Arc<dyn BucketStorage>,
    ) -> Self {
        Self {
            auth: AuthAppState::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                jwt_service.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
            ),
            user: UserAppState::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
                bucket_service.clone(),
            ),
            jwt_service,
        }
    }
}
