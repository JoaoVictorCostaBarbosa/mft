use crate::domain::errors::smtp_error::SmtpError;
use axum::async_trait;

#[async_trait]
pub trait SmtpService: Send + Sync + 'static {
    fn send_email(to: &str, subject: &str, html_body: &str) -> Result<(), SmtpError>;
}
