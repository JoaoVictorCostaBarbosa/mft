use thiserror::Error;

#[derive(Debug, Error)]
pub enum CriptoError {
    #[error("invalid credentials")]
    InvalidCredentials,
    
    #[error("error genereting hash")]
    HashError,

    #[error("error verifying hash")]
    VerifyError,
}
