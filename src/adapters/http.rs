use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    adapters::dto::{CreateUserReq, CreateUserResp, SpeakResp, UserResp},
    domain::user::{DomainError, Speak},
    usecases::user_service::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id/speak", get(user_speak))
        .with_state(state)
}

async fn health() -> &'static str {
    "I'm alive!"
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserReq>,
) -> axum::response::Response {
    match state.user_service.create_user(req.name).await {
        Ok(id) => (StatusCode::CREATED, Json(CreateUserResp { id })).into_response(),
        Err(e) => map_error(e),
    }
}

async fn get_all_users(State(state): State<AppState>) -> axum::response::Response {
    match state.user_service.get_all_users().await {
        Ok(users) => {
            let resp: Vec<UserResp> = users.into_iter().map(|u| {
                let greet = u.greet();
                UserResp {
                    id: u.id,
                    name: u.name,
                    active: u.active,
                    greet,
                }
            }).collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => map_error(e),
    }
}

async fn get_user(State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.user_service.get_user(id).await {
        Ok(u) => {
            let greet = u.greet();
            let resp = UserResp {
                id: u.id,
                name: u.name,
                active: u.active,
                greet,
            };
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => map_error(e),
    }
}

async fn user_speak(State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.user_service.get_user(id).await {
        Ok(u) => {
            let speak = match u.speak() {
                Ok(s) => s,
                Err(e) => return map_error(e),
            };
            let shout = match u.shout() {
                Ok(s) => s,
                Err(e) => return map_error(e),
            };
            (StatusCode::OK, Json(SpeakResp { speak, shout })).into_response()
        }
        Err(e) => map_error(e),
    }
}

// ---- error mapping (adapter responsibility) ----
fn map_error(e: DomainError) -> axum::response::Response {
    match e {
        DomainError::Validation(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        DomainError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
        DomainError::Unexpected(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
    }
}