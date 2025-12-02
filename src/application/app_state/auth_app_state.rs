use crate::{
    application::usecase::user::{create_user::CreateUser, login_user::LoginUser},
    domain::{
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, jwt::JwtProvider},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthAppUsate {
    pub create_user: Arc<CreateUser>,
    pub login_user: Arc<LoginUser>,
}

impl AuthAppUsate {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        cripto_service: Arc<dyn CriptoService>,
        jwt_service: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            create_user: Arc::new(CreateUser::new(
                user_repo.clone(),
                cripto_service.clone(),
                jwt_service.clone(),
            )),
            login_user: Arc::new(LoginUser::new(
                user_repo.clone(),
                cripto_service.clone(),
                jwt_service.clone(),
            )),
        }
    }
}
