use crate::interfaces::http::dtos::user_dto::{
    AuthResponseDTO, CreateUserRequestDTO, UserResponseDTO, LoginRequestDTO,
};
use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::handlers::user::create_user::create_user_handler,
        crate::interfaces::http::handlers::user::login_user::login_user_handler,
    ),
    components(
        schemas(
            CreateUserRequestDTO,
            UserResponseDTO,
            LoginRequestDTO,
            AuthResponseDTO,
        )
    ),
    tags(
        (name = "Auth", description = "User authentication"),
    ),
    modifiers(&SecurityAddon),
    info(title = "myFitTracker-API", version = "0.1.0")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
