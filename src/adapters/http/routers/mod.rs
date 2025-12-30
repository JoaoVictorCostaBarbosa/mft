use crate::{
    adapters::http::routers::{
        auth_router::auth_routers, exercise_router::exercise_routers, measurement_router::measurement_routers, user_router::user_routers
    },
    application::app_state::app_state::AppState,
};
use axum::Router;

pub mod auth_router;
pub mod exercise_router;
pub mod measurement_router;
pub mod user_router;

pub fn build_http() -> Router<AppState> {
    Router::new()
        .nest("/api", auth_routers())
        .nest("/api", user_routers())
        .nest("/api", measurement_routers())
        .nest("/api", exercise_routers())
}
