use thiserror::Error;

const MIN: f32 = 15.0;
const MAX: f32 = 250.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyPartMeasure(f32);

#[derive(Debug, Error)]
pub enum BodyPartMeasureError {
    #[error("out of range: min - {min}, max - {max}, received - {received}")]
    OutOfRange { min: f32, max: f32, received: f32 },
}

impl BodyPartMeasure {
    pub fn new(value: impl Into<f32>) -> Result<Self, BodyPartMeasureError> {
        let value = (value.into() * 10.0).round() / 10.0;

        if value < MIN || value > MAX {
            return Err(BodyPartMeasureError::OutOfRange {
                min: MIN,
                max: MAX,
                received: value,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
