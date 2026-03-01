use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};

use crate::infra::jwt::{verify_token, Claims};

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
