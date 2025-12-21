use crate::domain::{
    entities::user::User,
    enums::role::Role,
    errors::{domain_error::DomainError, permission_error::PermissionError},
    repositories::measurement_repository::MeasurementRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteMeasurement {
    pub measurement_repo: Arc<dyn MeasurementRepository>,
}

impl DeleteMeasurement {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), DomainError> {
        if current_user.role != Role::Admin {
            return Err(PermissionError::Forbidden)?;
        }

        self.measurement_repo.delete_measurement(id).await?;

        Ok(())
    }
}
