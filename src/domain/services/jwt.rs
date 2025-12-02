use crate::domain::{
    auth::token_data::{AccessTokenData, RefreshTokenData},
    enums::role::Role,
    errors::jwt_error::JwtError,
};

pub trait JwtProvider: Send + Sync + 'static {
    fn generate_access(&self, user_id: String, role: Role) -> Result<String, JwtError>;
    fn generate_refresh(&self, user_id: String) -> Result<String, JwtError>;
    fn verify_access(&self, token: &str) -> Result<AccessTokenData, JwtError>;
    fn verify_refresh(&self, token: &str) -> Result<RefreshTokenData, JwtError>;
    fn refresh_access(&self, refresh_token: &str, role: Role) -> Result<String, JwtError>;
}
