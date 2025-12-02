use crate::domain::enums::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}
