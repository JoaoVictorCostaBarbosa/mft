use crate::domain::errors::cripto_error::CriptoError;

pub trait RefreshTokenHasher: Send + Sync + 'static {
    fn hash(&self, token: &str) -> Result<String, CriptoError>;
}
