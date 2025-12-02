use crate::interfaces::http::dtos::user_dto::CreateUserRequestDTO;
use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::mappers::user_mapper::UserMappers,
};
use axum::{
    extract::{Json, State},
    http::status,
    response::IntoResponse,
};

#[utoipa::path{
    post,
    path = "/api/auth/register",
    request_body = CreateUserRequestDTO,
    responses(
        (status = 201, description = "create user", body = AuthResponseDTO),
        (status = 409, description = "email already used"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUserRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let user_create_dto = mapper.to_user_create_dto(user_data);

    match state.auth.create_user.execute(user_create_dto).await {
        Ok(user) => (
            status::StatusCode::CREATED,
            Json(mapper.to_auth_response_dto(user)),
        )
            .into_response(),
        Err(err) => err.into_response(),
    }
}
