use crate::domain::errors::smtp_error::SmtpError;
use axum::async_trait;

#[async_trait]
pub trait SmtpService: Send + Sync + 'static {
    async fn send_email(&self, to: &str, subject: &str, code: &str) -> Result<(), SmtpError>;
}
