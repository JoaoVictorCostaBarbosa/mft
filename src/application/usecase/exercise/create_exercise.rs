use crate::{
    application::dtos::exercise::create_exercise::CreateExerciseRequest,
    domain::{
        entities::{exercise::Exercise, user::User},
        enums::role::Role,
        errors::domain_error::DomainError,
        repositories::exercise_repository::ExerciseRepository,
    },
};
use std::sync::Arc;

pub struct CreateExercise {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl CreateExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        request: CreateExerciseRequest,
        current_user: User,
    ) -> Result<Exercise, DomainError> {
        let user_id = match current_user.role {
            Role::Admin => None,
            _ => Some(current_user.id),
        };

        let exercise = Exercise::new(
            user_id,
            request.name,
            request.exercise_type,
            request.equipment,
            request.muscle_group,
        )?;

        self.exercise_repo.create_exercise(&exercise).await?;

        Ok(exercise)
    }
}
