use crate::domain::enums::{
    equipment::Equipment, exercise_type::ExerciseType, muscle_group::MuscleGroup,
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ExerciseUpdateFields {
    pub name: Option<String>,
    pub exercise_type: Option<ExerciseType>,
    pub equipment: Option<Equipment>,
    pub muscle_group: Option<MuscleGroup>,
}

#[derive(Debug, Default)]
pub struct ExerciseFilterFields {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub equipment: Option<Equipment>,
    pub exercise_type: Option<ExerciseType>,
    pub muscle_group: Option<MuscleGroup>,
}
