use crate::domain::value_objects::name_vo::NameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExerciseError {
    #[error("invalid name: {0}")]
    NameInvalid(#[from] NameError),

    #[error("invalid search criteria")]
    InvalidSearchCriteria,
}
