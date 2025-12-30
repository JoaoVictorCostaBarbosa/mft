use crate::{
    application::usecase::exercise::{
        create_exercise::CreateExercise, delete_exercise::DeleteExercise,
        get_exercise_by_id::GetExerciseById, read_exercises::ReadExercises,
        search_exercises::SearchExercises, soft_delete_exercise::SoftDeleteExercise,
        update_exercise::UpdateExercise,
    },
    domain::repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ExerciseAppState {
    pub create: Arc<CreateExercise>,
    pub get_by_id: Arc<GetExerciseById>,
    pub read: Arc<ReadExercises>,
    pub search: Arc<SearchExercises>,
    pub update: Arc<UpdateExercise>,
    pub soft_delete: Arc<SoftDeleteExercise>,
    pub delete: Arc<DeleteExercise>,
}

impl ExerciseAppState {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self {
            create: Arc::new(CreateExercise::new(exercise_repo.clone())),
            get_by_id: Arc::new(GetExerciseById::new(exercise_repo.clone())),
            read: Arc::new(ReadExercises::new(exercise_repo.clone())),
            search: Arc::new(SearchExercises::new(exercise_repo.clone())),
            update: Arc::new(UpdateExercise::new(exercise_repo.clone())),
            soft_delete: Arc::new(SoftDeleteExercise::new(exercise_repo.clone())),
            delete: Arc::new(DeleteExercise::new(exercise_repo.clone())),
        }
    }
}
