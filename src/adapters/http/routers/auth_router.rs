use crate::{
    application::app_state::app_state::AppState,
    adapters::http::handlers::user::{
        create_user::create_user_handler, login_user::login_user_handler,
        verify_user::verify_user_handler,
    },
};
use axum::{Router, routing::post};

pub fn auth_routers() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(create_user_handler))
        .route("/auth/verify", post(verify_user_handler))
        .route("/auth/login", post(login_user_handler))
}
