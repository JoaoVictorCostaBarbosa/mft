use crate::{
    domain::{
        entities::user::User,
        errors::user_error::UserError,
        value_objects::{email_vo::Email, name_vo::Name},
    },
    infrastructure::repositories::enums_db::role_db::RoleDb,
};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: RoleDb,
    pub url_img: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl UserModel {
    pub fn to_domain(&self) -> Result<User, UserError> {
        Ok(User {
            id: self.id,
            name: Name::new(self.name.clone())?,
            email: Email::new(self.email.clone())?,
            password: self.password.clone(),
            role: self.role.clone().into(),
            url_img: self.url_img.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        })
    }
}
