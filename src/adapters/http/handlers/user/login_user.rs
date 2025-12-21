use crate::{
    adapters::http::{dtos::user_dto::LoginRequestDTO, errors::http_error::HttpError, mappers::user_mapper::UserMappers}, application::app_state::app_state::AppState
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    post,
    path = "/api/auth/login",
    request_body = LoginRequestDTO,
    responses(
        (status = 200, description = "login successful", body = AuthResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(user_data): Json<LoginRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;

    let data = mapper.to_login_request(user_data);

    match state.auth.login_user.execute(data).await {
        Ok(user) => {
            let user_response = mapper.to_auth_response_dto(user);
            (StatusCode::OK, Json(user_response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
