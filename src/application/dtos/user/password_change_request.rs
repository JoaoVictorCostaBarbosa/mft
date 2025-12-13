use uuid::Uuid;

pub struct PasswordChangeRequest {
    pub id: Option<Uuid>,
    pub password: String,
    pub code: u32,
}
