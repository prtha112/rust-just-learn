use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::DomainError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}

pub fn sign_token(user_id: i64, username: String) -> Result<String, DomainError> {
    let secret = env::var("JWT_SECRET")
        .map_err(|_| DomainError::Unexpected("JWT_SECRET not set".into()))?;

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| DomainError::Unexpected(e.to_string()))?
        .as_secs() as usize
        + 24 * 3600;

    let claims = Claims { sub: user_id, username, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| DomainError::Unexpected(e.to_string()))
}

pub fn verify_token(token: &str) -> Result<Claims, DomainError> {
    let secret = env::var("JWT_SECRET")
        .map_err(|_| DomainError::Unexpected("JWT_SECRET not set".into()))?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| DomainError::Unauthorized)
}
