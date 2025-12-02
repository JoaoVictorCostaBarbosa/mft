use crate::domain::{entities::user::User, errors::domain_error::DomainError};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn get_all_users(&self) -> Result<Vec<User>, DomainError>;
    async fn update_user_name(&self, name: String, user_id: Uuid) -> Result<(), DomainError>;
    async fn update_user_email(&self, email: String, user_id: Uuid) -> Result<(), DomainError>;
    async fn update_user_password(&self, passsword: String, user_id: Uuid) -> Result<(), DomainError>;
    async fn update_user_avatar(&self, avatar_url: String, user_id: Uuid) -> Result<(), DomainError>;
    async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}
