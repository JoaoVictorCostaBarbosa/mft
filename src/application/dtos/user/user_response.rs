use crate::domain::entities::user::User;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub url_img: Option<String>,
}

impl UserResponse {
    pub fn to_response(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name.value().to_string(),
            email: user.email.value().to_string(),
            url_img: user.url_img.clone(),
        }
    }
}
