use crate::domain::{
    entities::{measurement::Measurement, user::User},
    errors::{
        domain_error::DomainError, permission_error::PermissionError,
        repository_error::RepositoryError,
    },
    repositories::measurement_repository::MeasurementRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetMeasurementById {
    pub measurement_repo: Arc<dyn MeasurementRepository>,
}

impl GetMeasurementById {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<Measurement, DomainError> {
        let measurement = self.measurement_repo.get_measurement_by_id(id).await?;

        if measurement.user_id != current_user.id {
            return Err(PermissionError::Forbidden)?;
        }

        if measurement.deleted_at.is_some() {
            return Err(RepositoryError::NotFound(
                "measurement not found".to_string(),
            ))?;
        }

        Ok(measurement)
    }
}
