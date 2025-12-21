use crate::{
    adapters::http::{
        dtos::measurement_dto::CreateMeasurementDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::measurement_mapper::MeasurementMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn create_measurement_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<CreateMeasurementDTO>,
) -> impl IntoResponse {
    match state
        .measurement
        .create
        .execute(MeasurementMapper::dto_to_request(request), current_user)
        .await
    {
        Ok(measurement) => (
            StatusCode::CREATED,
            Json(MeasurementMapper::domain_to_response(measurement)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
