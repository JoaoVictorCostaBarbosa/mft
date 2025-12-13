use uuid::Uuid;

pub struct EmailChangeRequest {
    pub id: Option<Uuid>,
    pub email: String,
    pub code: u32,
}
