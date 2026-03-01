use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    adapters::auth_middleware::ApiKey,
    adapters::dto_user::{CreateUserReq, CreateUserResp, LoginReq, LoginResp, SpeakResp, UpdateUserReq, UserResp},
    domain::user::Speak,
    infra::jwt::{sign_token, Claims},
};

use super::AppState;

pub async fn create_user(
    _api_key: ApiKey,
    State(state): State<AppState>,
    Json(req): Json<CreateUserReq>,
) -> axum::response::Response {
    match state.user_service.create_user(req.username.clone(), req.password).await {
        Ok(id) => {
            tracing::info!(user_id = id, username = %req.username, "user created: {:#?}", req.username);
            (StatusCode::CREATED, Json(CreateUserResp { id })).into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn get_all_users(_claims: Claims, State(state): State<AppState>) -> axum::response::Response {
    match state.user_service.get_all_users().await {
        Ok(users) => {
            tracing::info!(count = users.len(), "fetched users: {:#?}", &users[..users.len().min(5)]);
            let resp: Vec<UserResp> = users.into_iter().map(UserResp::from).collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => super::map_error(e),
    }
}

pub async fn get_user(_claims: Claims, State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.user_service.get_user(id).await {
        Ok(u) => {
            tracing::info!(user_id = u.id, username = %u.username, active = u.active, "fetched user {:#?}", u);
            (StatusCode::OK, Json(UserResp::from(u))).into_response()
        },
        Err(e) => super::map_error(e),
    }
}

pub async fn update_user(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserReq>,
) -> axum::response::Response {
    match state.user_service.update_user(id, req.username, req.password).await {
        Ok(u) => {
            tracing::info!(user_id = u.id, username = %u.username, active = u.active, "updated user {:#?}", u);
            (StatusCode::OK, Json(UserResp::from(u))).into_response()
        },
        Err(e) => super::map_error(e),
    }
}

pub async fn user_speak(_claims: Claims, State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.user_service.get_user(id).await {
        Ok(u) => {
            let speak = match u.speak() {
                Ok(s) => s,
                Err(e) => return super::map_error(e),
            };
            let shout = match u.shout() {
                Ok(s) => s,
                Err(e) => return super::map_error(e),
            };
            (StatusCode::OK, Json(SpeakResp { speak, shout })).into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn delete_user(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    match state.user_service.delete_user(id).await {
        Ok(_) => {
            tracing::info!(user_id = id, "deleted user id = {:#?}", id);
            (StatusCode::NO_CONTENT, Json(())).into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> axum::response::Response {
    match state.user_service.login(req.username, req.password).await {
        Ok(u) => {
            match sign_token(u.id, u.username.clone()) {
                Ok(token) => {
                    tracing::info!(user_id = u.id, username = %u.username, "user logged in");
                    (StatusCode::OK, Json(LoginResp { token })).into_response()
                }
                Err(e) => super::map_error(e),
            }
        },
        Err(e) => super::map_error(e),
    }
}
