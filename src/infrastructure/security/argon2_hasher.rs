use crate::domain::errors::cripto_error::CriptoError;
use crate::domain::services::cripto::CriptoService;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

pub struct Argon2Hasher;

impl CriptoService for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, CriptoError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| CriptoError::HashError)?
            .to_string();

        Ok(hash)
    }

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, CriptoError> {
        let parsed = PasswordHash::new(password_hash).map_err(|_| CriptoError::VerifyError)?;

        let argon = Argon2::default();

        if argon.verify_password(password.as_bytes(), &parsed).is_ok() {
            return Ok(true);
        }

        Ok(false)
    }
}
