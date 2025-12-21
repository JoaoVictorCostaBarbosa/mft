use crate::{
    adapters::http::{
        errors::http_error::HttpError,
        extractors::{current_user::CurrentUser, image_file::ImageFile},
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn update_avatar_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    ImageFile(file): ImageFile,
) -> impl IntoResponse {
    let mapper = UserMappers;

    match state.user.update_avatar.execute(file, current_user).await {
        Ok(user) => {
            let response = mapper.to_user_response_dto(user);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
