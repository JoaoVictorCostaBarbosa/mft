use crate::domain::enums::{
    equipment::Equipment, exercise_type::ExerciseType, muscle_group::MuscleGroup,
};

pub struct CreateExerciseRequest {
    pub name: String,
    pub exercise_type: ExerciseType,
    pub equipment: Equipment,
    pub muscle_group: MuscleGroup,
}
