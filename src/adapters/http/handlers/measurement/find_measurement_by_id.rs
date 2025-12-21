use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::measurement_mapper::MeasurementMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn find_measurement_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.measurement.get_by_id.execute(id, current_user).await {
        Ok(measurement) => (
            StatusCode::OK,
            Json(MeasurementMapper::domain_to_response(measurement)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
