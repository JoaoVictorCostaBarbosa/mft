use thiserror::Error;

const MIN_WEIGHT: f32 = 0.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Weight(f32);

#[derive(Debug, Error)]
pub enum WeightError {
    #[error("weight has to be positive")]
    Negative,
}

impl Weight {
    pub fn new(value: impl Into<f32>) -> Result<Self, WeightError> {
        let value = value.into();
        
        if value < MIN_WEIGHT {
            return Err(WeightError::Negative);
        }
        
        Ok(Self(value))
    }
    
    pub fn value(&self) -> f32 {
        self.0
    }
}