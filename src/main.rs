mod api_doc;
mod application;
mod db;
mod domain;
mod infrastructure;
mod interfaces;
use crate::{
    api_doc::ApiDoc,
    application::app_state::app_state::AppState,
    domain::services::{cripto::CriptoService, jwt::JwtProvider},
    infrastructure::{
        repositories::postgres::repository_bundle::RepositoryBundle,
        security::{argon2_hasher::Argon2Hasher, jwt::jwt_token_service::JwtService},
    },
    interfaces::http::routers::build_http,
};
use axum::Router;
use db::create_pool;
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn root() -> &'static str {
    "Servidor está rodando"
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    println!("INFO sqlx::pool: connection established");

    let repos = RepositoryBundle::new(pool.clone());

    let cripto_service: Arc<dyn CriptoService> = Arc::new(Argon2Hasher {});

    let access_secret = env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not set");
    let refresh_secret = env::var("SECRET_REFRESH_KEY").expect("SECRET_REFRESH_KEY not set");

    let jwt_service: Arc<dyn JwtProvider> =
        Arc::new(JwtService::new(access_secret, refresh_secret));

    let app_state = AppState::new(repos, cripto_service, jwt_service);

    let app = Router::new()
        .route("/", axum::routing::get(root))
        .merge(build_http::<AppState>())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("INFO server: running on {}", addr);
    
    println!("API documentation in: http://localhost:3000/swagger-ui");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
