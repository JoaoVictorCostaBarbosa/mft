use crate::domain::{
    entities::user::User,
    enums::role::Role,
    errors::{domain_error::DomainError, permission_error::PermissionError},
    repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteExercise {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl DeleteExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), DomainError> {
        if current_user.role != Role::Admin {
            return Err(PermissionError::Forbidden.into());
        }

        self.exercise_repo.delete_exercise(id).await?;

        Ok(())
    }
}
