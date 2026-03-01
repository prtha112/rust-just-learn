use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};

use crate::infra::jwt::{verify_token, Claims};

pub struct ApiKey;

#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let expected = std::env::var("API_KEY")
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "API_KEY not set").into_response())?;

        let provided = parts
            .headers
            .get("X-Api-Key")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing X-Api-Key header").into_response())?;

        if provided != expected {
            return Err((StatusCode::UNAUTHORIZED, "invalid api key").into_response());
        }

        Ok(ApiKey)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (StatusCode::UNAUTHORIZED, "missing authorization header").into_response()
            })?;

        verify_token(token).map_err(|_| {
            (StatusCode::UNAUTHORIZED, "invalid or expired token").into_response()
        })
    }
}
