use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use tower_http::trace::TraceLayer;

use crate::{
    adapters::dto_user::{CreateUserReq, CreateUserResp, SpeakResp, UserResp, UpdateUserReq},
    adapters::dto_catagory::{CreateCatagoryReq, CreateCatagoryResp, CatagoryResp, UpdateCatagoryReq},
    domain::DomainError,
    domain::user::Speak,
    infra::http_trace::{OtelOnResponse, record_http_request},
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
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/users/:id/speak", get(user_speak))
        .route("/catagories", post(create_catagory))
        .route("/catagories/:id", put(update_catagory))
        .route("/catagories", get(get_all_catagories))
        .route("/catagories/:id", get(get_catagory))
        .route("/catagories/:id", delete(delete_catagory))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new()
                        .level(tracing::Level::INFO),
                )
                .on_response(OtelOnResponse),
        )
}

async fn health() -> &'static str {
    record_http_request("GET", "/health", 200, 0.0);
    "I'm alive!"
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserReq>,
) -> axum::response::Response {
    match state.user_service.create_user(req.username.clone(), req.password).await {
        Ok(id) => {
            tracing::info!(user_id = id, username = %req.username, "user created: {:#?}", req.username.clone());
            (StatusCode::CREATED, Json(CreateUserResp { id })).into_response()
        }
        Err(e) => {
            map_error(e)
        }
    }
}

async fn get_all_users(State(state): State<AppState>) -> axum::response::Response {
    match state.user_service.get_all_users().await {
        Ok(users) => {
            tracing::info!(count = users.len(), "fetched users: {:#?}", &users[..users.len().min(5)]);
            let resp: Vec<UserResp> = users.into_iter().map(|u| {
                let greet = u.greet();
                UserResp { id: u.id, username: u.username, password: u.password, active: u.active, greet }
            }).collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => {
            map_error(e)
        }
    }
}

async fn get_user(State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.user_service.get_user(id).await {
        Ok(u) => {
            tracing::info!(user_id = u.id, username = %u.username, active = u.active, "fetched user {:#?}", u);
            let greet = u.greet();
            let resp = UserResp { id: u.id, username: u.username, password: u.password, active: u.active, greet };
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => {
            map_error(e)
        }
    }
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserReq>,
) -> axum::response::Response {
    match state.user_service.update_user(id, req.username, req.password).await {
        Ok(u) => {
            tracing::info!(user_id = u.id, username = %u.username, active = u.active, "updated user {:#?}", u);
            let greet = u.greet();
            let resp = UserResp { id: u.id, username: u.username, password: u.password, active: u.active, greet };
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

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    match state.user_service.delete_user(id).await {
        Ok(_) => {
            tracing::info!(user_id = id, "deleted user id = {:#?}", id);
            (StatusCode::NO_CONTENT, Json(())).into_response()
        }
        Err(e) => map_error(e),
    }
}

async fn create_catagory(
    State(state): State<AppState>,
    Json(req): Json<CreateCatagoryReq>,
) -> axum::response::Response {
    match state.catagory_service.create(req.name.clone()).await {
        Ok(id) => {
            tracing::info!(catagory_id = id, name = %req.name, "catagory created");
            (StatusCode::CREATED, Json(CreateCatagoryResp { id })).into_response()
        }
        Err(e) => {
            map_error(e)
        }
    }
}

async fn get_all_catagories(State(state): State<AppState>) -> axum::response::Response {
    match state.catagory_service.get_all_catagories().await {
        Ok(catagories) => {
            tracing::info!(
                count = catagories.len(),
                "fetched catagories: {:#?}",
                &catagories[..catagories.len().min(5)]
            );
            let resp: Vec<CatagoryResp> = catagories
                .into_iter()
                .map(|c| CatagoryResp {
                    id: c.id,
                    name: c.name,
                    active: c.active,
                })
                .collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => {
            map_error(e)
        }
    }
}

async fn get_catagory(State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.catagory_service.get_by_id(id).await {
        Ok(Some(c)) => {
            tracing::info!(catagory_id = c.id, name = %c.name, active = c.active, "fetched catagory");
            let resp = CatagoryResp { id: c.id, name: c.name, active: c.active };
            (StatusCode::OK, Json(resp)).into_response()
        },
        Ok(None) => {
            tracing::warn!(catagory_id = id, "catagory not found");
            (StatusCode::NOT_FOUND, "not found").into_response()
        }
        Err(e) => {
            map_error(e)
        }
    }
}

async fn update_catagory(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCatagoryReq>,
) -> axum::response::Response {
    match state.catagory_service.update(id, req.name).await {
        Ok(c) => {
            tracing::info!(catagory_id = c.id, name = %c.name, active = c.active, "updated catagory");
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

async fn delete_catagory(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    match state.catagory_service.delete(id).await {
        Ok(_) => {
            tracing::info!(catagory_id = id, "deleted catagory id = {:#?}", id);
            (StatusCode::NO_CONTENT, Json(())).into_response()
        }
        Err(e) => map_error(e),
    }
}

// ---- error mapping (adapter responsibility) ----
fn map_error(e: DomainError) -> axum::response::Response {
    match e {
        DomainError::Validation(msg) => {
            tracing::warn!(error = %msg, "validation error");
            (StatusCode::BAD_REQUEST, msg).into_response()
        },
        DomainError::NotFound => {
            tracing::warn!(error = "not found", "not found");
            (StatusCode::NOT_FOUND, "not found").into_response()
        },
        DomainError::Unexpected(msg) => {
            tracing::warn!(error = %msg, "unexpected error");
            (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        },
    }
}