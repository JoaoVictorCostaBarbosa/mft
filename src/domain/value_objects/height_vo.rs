use thiserror::Error;

const MIN_HEIGHT: f32 = 100.0;
const MAX_HEIGHT: f32 = 272.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Height(f32);

#[derive(Debug, Error)]
pub enum HeightError {
    #[error("height too small")]
    TooSmall,
    
    #[error("height too large")]
    TooLarge,
}

impl Height {
    pub fn new(value: impl Into<f32>) -> Result<Self, HeightError> {
        let value = value.into();

        if value < MIN_HEIGHT {
            return Err(HeightError::TooSmall);
        }

        if value > MAX_HEIGHT {
            return Err(HeightError::TooLarge);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
