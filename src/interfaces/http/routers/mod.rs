use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::routers::auth_route::user_routers,
};
use axum::Router;

pub mod auth_route;

pub fn build_http<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    AppState: axum::extract::FromRef<S>,
{
    Router::new().nest("/api", user_routers::<S>())
}
