use crate::domain::{
    commands::exercise_commands::ExerciseUpdateFields,
    entities::user::User,
    enums::role::Role,
    errors::{domain_error::DomainError, exercise_error::ExerciseError, user_error::UserError},
    repositories::exercise_repository::ExerciseRepository,
    value_objects::name_vo::Name,
};
use std::sync::Arc;

pub struct UpdateExercise {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl UpdateExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        fields: ExerciseUpdateFields,
    ) -> Result<(), DomainError> {
        if fields.is_empty() {
            return Err(ExerciseError::InvalidFieldsCriteria.into());
        }

        match fields.name.clone() {
            Some(n) => {
                Name::new(n).map_err(UserError::from)?;
            }
            None => {}
        }

        let user_id = match current_user.role {
            Role::Admin => None,
            Role::User => Some(current_user.id),
        };

        self.exercise_repo.update_exercise(fields, user_id).await?;

        Ok(())
    }
}
