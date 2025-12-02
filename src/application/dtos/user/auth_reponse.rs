use crate::application::dtos::user::user_response::UserResponse;

#[derive(Debug)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access: String,
    pub refresh: String,
}

impl AuthResponse {
    pub fn new(user: UserResponse, access: String, refresh: String) -> Self {
        Self {
            user,
            access,
            refresh,
        }
    }
}
