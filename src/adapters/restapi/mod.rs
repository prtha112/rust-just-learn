use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::{
    domain::DomainError,
    infra::http_trace::{OtelOnResponse, record_http_request},
    usecases::{category_service::CategoryService, user_service::UserService, product_service::ProductService},
};

mod user;
mod category;
mod product;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub category_service: CategoryService,
    pub product_service: ProductService,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/login", post(user::login_user))
        .route("/users", post(user::create_user))
        .route("/users", get(user::get_all_users))
        .route("/users/:id", get(user::get_user))
        .route("/users/:id", put(user::update_user))
        .route("/users/:id", delete(user::delete_user))
        .route("/users/:id/speak", get(user::user_speak))
        .route("/categories", post(category::create_category))
        .route("/categories", get(category::get_all_categories))
        .route("/categories/:id", get(category::get_category))
        .route("/categories/:id", put(category::update_category))
        .route("/categories/:id", delete(category::delete_category))
        .route("/products", post(product::create_product))
        .route("/products/:id", get(product::get_product))
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
        DomainError::Unauthorized => {
            tracing::warn!(error = "unauthorized", "unauthorized");
            (StatusCode::UNAUTHORIZED, "unauthorized").into_response()
        },
        DomainError::Unexpected(msg) => {
            tracing::warn!(error = %msg, "unexpected error");
            (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        },
    }
}
