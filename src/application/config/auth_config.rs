#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub refresh_token_exp_time: i64,
}

impl AuthConfig {
    pub fn new(refresh_token_exp_time: i64) -> Self {
        Self {
            refresh_token_exp_time,
        }
    }
}
