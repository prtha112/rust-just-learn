use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    adapters::dto_category::{CategoryResp, CreateCategoryReq, CreateCategoryResp, UpdateCategoryReq},
    infra::jwt::Claims,
};

use super::AppState;

pub async fn create_category(
    _claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<CreateCategoryReq>,
) -> axum::response::Response {
    match state.category_service.create(req.name.clone()).await {
        Ok(id) => {
            tracing::info!(category_id = id, name = %req.name, "category created");
            (StatusCode::CREATED, Json(CreateCategoryResp { id })).into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn get_all_categories(_claims: Claims, State(state): State<AppState>) -> axum::response::Response {
    match state.category_service.get_all_categories().await {
        Ok(categories) => {
            tracing::info!(
                count = categories.len(),
                "fetched categories: {:#?}",
                &categories[..categories.len().min(5)]
            );
            let resp: Vec<CategoryResp> = categories
                .into_iter()
                .map(|c| CategoryResp { id: c.id, name: c.name, active: c.active })
                .collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => super::map_error(e),
    }
}

pub async fn get_category(_claims: Claims, State(state): State<AppState>, Path(id): Path<i64>) -> axum::response::Response {
    match state.category_service.get_by_id(id).await {
        Ok(Some(c)) => {
            tracing::info!(category_id = c.id, name = %c.name, active = c.active, "fetched category");
            (StatusCode::OK, Json(CategoryResp { id: c.id, name: c.name, active: c.active })).into_response()
        },
        Ok(None) => {
            tracing::warn!(category_id = id, "category not found");
            (StatusCode::NOT_FOUND, "not found").into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn update_category(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCategoryReq>,
) -> axum::response::Response {
    match state.category_service.update(id, req.name).await {
        Ok(c) => {
            tracing::info!(category_id = c.id, name = %c.name, active = c.active, "updated category");
            (StatusCode::OK, Json(CategoryResp { id: c.id, name: c.name, active: c.active })).into_response()
        },
        Err(e) => super::map_error(e),
    }
}

pub async fn delete_category(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    match state.category_service.delete(id).await {
        Ok(_) => {
            tracing::info!(category_id = id, "deleted category id = {:#?}", id);
            (StatusCode::NO_CONTENT, Json(())).into_response()
        }
        Err(e) => super::map_error(e),
    }
}
