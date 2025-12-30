use crate::{
    application::{
        app_state::{
            auth_app_state::AuthAppState, exercise_app_state::ExerciseAppState,
            measurement_app_state::MeasurementAppState, user_app_state::UserAppState,
        },
        config::auth_config::AuthConfig,
        interfaces::{
            pending_change_repository::PendingChangesRepository,
            pending_user_repository::PendingUserRepository,
        },
    },
    domain::{
        repositories::{
            exercise_repository::ExerciseRepository, measurement_repository::MeasurementRepository,
            refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
        },
        services::{
            bucket_storage::BucketStorage, cripto::CriptoService, jwt::JwtProvider, refresh_token_hasher::RefreshTokenHasher, smtp::SmtpService
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthAppState,
    pub user: UserAppState,
    pub measurement: MeasurementAppState,
    pub exercise: ExerciseAppState,
    pub jwt_service: Arc<dyn JwtProvider>,
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        measurement_repo: Arc<dyn MeasurementRepository>,
        exercise_repo: Arc<dyn ExerciseRepository>,
        cripto_service: Arc<dyn CriptoService>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        jwt_service: Arc<dyn JwtProvider>,
        smtp_service: Arc<dyn SmtpService>,
        bucket_service: Arc<dyn BucketStorage>,
        refresh_exp_days: i64,
    ) -> Self {
        Self {
            auth: AuthAppState::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                refresh_token_repo.clone(),
                cripto_service.clone(),
                jwt_service.clone(),
                hash_service.clone(),
                smtp_service.clone(),
                Arc::new(AuthConfig::new(refresh_exp_days)),
            ),
            user: UserAppState::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
                bucket_service.clone(),
            ),
            measurement: MeasurementAppState::new(measurement_repo.clone()),
            exercise: ExerciseAppState::new(exercise_repo.clone()),
            jwt_service,
        }
    }
}
