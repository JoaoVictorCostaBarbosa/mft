use crate::{
    application::app_state::auth_app_state::AuthAppUsate,
    domain::{
        services::{cripto::CriptoService, jwt::JwtProvider},
    },
    infrastructure::repositories::postgres::repository_bundle::RepositoryBundle,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthAppUsate,
}

impl AppState {
    pub fn new(
        repos: RepositoryBundle,
        cripto_service: Arc<dyn CriptoService>,
        jwt_service: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            auth: AuthAppUsate::new(repos.user_repo, cripto_service, jwt_service),
        }
    }
}
