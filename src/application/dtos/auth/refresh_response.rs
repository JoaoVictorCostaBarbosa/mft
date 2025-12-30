#[derive(Debug)]
pub struct RefreshResponse {
    pub access: String,
    pub refresh: String,
}

impl RefreshResponse {
    pub fn new(access: String, refresh: String) -> Self {
        Self { access, refresh }
    }
}
