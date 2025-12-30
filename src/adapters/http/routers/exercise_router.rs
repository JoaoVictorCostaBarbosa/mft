use crate::{
    adapters::http::handlers::exercise::{
        create_exercise::create_exercise_handler, delete_exercise::delete_exercise_handler,
        get_exercise_by_id::get_exercise_by_id_handler, read_exercises::read_exercises_handler,
        search_equipment::search_equipment_handler,
        search_exercise_type::search_exercise_type_handler,
        search_muscle_group::search_myscle_group_exercise,
        soft_delete_exercise::soft_delete_exercise_handler,
        update_exercise::update_exercise_handler,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

pub fn exercise_routers() -> Router<AppState> {
    Router::new()
        .route("/exercises", post(create_exercise_handler))
        .route("/exercises", get(read_exercises_handler))
        .route("/exercises/:id", get(get_exercise_by_id_handler))
        .route(
            "/exercises/type/:exercise_type",
            get(search_exercise_type_handler),
        )
        .route(
            "/exercises/equipment/:equipment",
            get(search_equipment_handler),
        )
        .route(
            "/exercises/muscle-group/:muscle_group",
            get(search_myscle_group_exercise),
        )
        .route("/exercises", put(update_exercise_handler))
        .route(
            "/exercises/:id/soft-delete",
            patch(soft_delete_exercise_handler),
        )
        .route("/exercises/:id", delete(delete_exercise_handler))
}
