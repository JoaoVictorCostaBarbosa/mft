use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmtpError {
    #[error("email not find")]
    NotFind
}
