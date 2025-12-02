use std::env;

use super::claims::{AccessClaims, RefreshClaims};
use crate::domain::{
    auth::token_data::{AccessTokenData, RefreshTokenData},
    enums::role::Role,
    errors::jwt_error::JwtError,
    services::jwt::JwtProvider,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct JwtService {
    access_secret: String,
    refresh_secret: String,
}

impl JwtService {
    pub fn new(access_secret: impl Into<String>, refresh_secret: impl Into<String>) -> Self {
        Self {
            access_secret: access_secret.into(),
            refresh_secret: refresh_secret.into(),
        }
    }
}

impl JwtProvider for JwtService {
    fn generate_access(&self, user_id: String, role: Role) -> Result<String, JwtError> {
        let minutes: i64 = (env::var("ACCESS_MINUTES").expect("ACCESS_MINUTES not set"))
            .parse()
            .expect("conversion failure");

        let exp = (Utc::now() + Duration::minutes(minutes)).timestamp() as usize;

        let claims = AccessClaims {
            sub: user_id,
            role,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )
        .map_err(JwtError::from)
    }

    fn generate_refresh(&self, user_id: String) -> Result<String, JwtError> {
        let days: i64 = (env::var("REFRESH_DAYS").expect("REFRESH_DAYS not set"))
            .parse()
            .expect("conversion failure");

        let exp = (Utc::now() + Duration::days(days)).timestamp() as usize;

        let claims = RefreshClaims { sub: user_id, exp };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes()),
        )
        .map_err(JwtError::from)
    }

    fn verify_access(
        &self,
        token: &str,
    ) -> Result<crate::domain::auth::token_data::AccessTokenData, JwtError> {
        let data = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(JwtError::from)?;

        Ok(AccessTokenData {
            user_id: data.claims.sub,
            role: data.claims.role,
        })
    }

    fn verify_refresh(
        &self,
        token: &str,
    ) -> Result<crate::domain::auth::token_data::RefreshTokenData, JwtError> {
        let data = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(JwtError::from)?;

        Ok(RefreshTokenData {
            user_id: data.claims.sub,
        })
    }

    fn refresh_access(&self, refresh_token: &str, role: Role) -> Result<String, JwtError> {
        let refresh_data = self.verify_refresh(refresh_token)?;
        self.generate_access(refresh_data.user_id, role)
    }
}
