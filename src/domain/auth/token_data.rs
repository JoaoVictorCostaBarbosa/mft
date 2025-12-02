use crate::domain::enums::role::Role;

pub struct AccessTokenData {
    pub user_id: String,
    pub role: Role,
}

pub struct RefreshTokenData {
    pub user_id: String,
}
