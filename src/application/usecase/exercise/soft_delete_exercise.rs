use crate::domain::{
    entities::user::User, errors::domain_error::DomainError,
    repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SoftDeleteExercise {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl SoftDeleteExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), DomainError> {
        self.exercise_repo
            .soft_delete_exercise(id, current_user.id)
            .await?;

        Ok(())
    }
}
