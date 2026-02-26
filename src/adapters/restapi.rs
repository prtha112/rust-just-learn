use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    adapters::dto::{CreateUserReq, CreateUserResp, SpeakResp, UserResp, CreateCatagoryReq, CreateCatagoryResp, CatagoryResp, UpdateCatagoryReq},
    domain::catagory::CatagoryRepository,
    domain::user::{DomainError, Speak},
    usecases::user_service::UserService,
    usecases::catagory_service::CatagoryService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub catagory_service: CatagoryService,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id/speak", get(user_speak))
        .route("/catagories", post(create_catagory))
        .route("/catagories/:id", put(update_catagory))
        .route("/catagories", get(get_all_catagories))
        .route("/catagories/:id", get(get_catagory))
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

async fn create_catagory(
    State(state): State<AppState>,
    Json(req): Json<CreateCatagoryReq>,
) -> axum::response::Response {
    match state.catagory_service.create(req.name).await {
        Ok(id) => (StatusCode::CREATED, Json(CreateCatagoryResp { id })).into_response(),
        Err(e) => map_error(e),
    }
}

async fn get_all_catagories(State(state): State<AppState>) -> axum::response::Response {
    match state.catagory_service.get_all_catagories().await {
        Ok(catagories) => {
            let resp: Vec<CatagoryResp> = catagories.into_iter().map(|c| {
                CatagoryResp {
                    id: c.id,
                    name: c.name,
                    active: c.active,
                }
            }).collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => map_error(e),
    }
}

async fn get_catagory(State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.catagory_service.get_by_id(id).await {
        Ok(Some(c)) => {
            let resp = CatagoryResp {
                id: c.id,
                name: c.name,
                active: c.active,
            };
            (StatusCode::OK, Json(resp)).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => map_error(e),
    }
}

async fn update_catagory(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCatagoryReq>,
) -> axum::response::Response {
    match state.catagory_service.update(id, req.name).await {
        Ok(c) => {
            let resp = CatagoryResp {
                id: c.id,
                name: c.name,
                active: c.active,
            };
            (StatusCode::OK, Json(resp)).into_response()
        },
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