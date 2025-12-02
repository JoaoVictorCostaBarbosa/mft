use crate::domain::{
    enums::role::Role,
    errors::user_error::UserError,
    value_objects::{email_vo::Email, name_vo::Name},
};
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Name,
    pub email: Email,
    pub password: String,
    pub role: Role,
    pub url_img: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Result<Self, UserError> {
        let now: NaiveDateTime = Local::now().naive_local();

        Ok(Self {
            id: Uuid::new_v4(),
            name: Name::new(name)?,
            email: Email::new(email)?,
            password: password_hash,
            role: Role::User,
            url_img: None,
            created_at: now,
            updated_at: None,
            deleted_at: None,
        })
    }
}
