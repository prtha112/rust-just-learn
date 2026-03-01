use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse},
    Json,
};

use crate::{
    adapters::dto_product::{CreateProductReq, CreateProductResp, ProductResp},
    domain::product::Product,
    infra::jwt::Claims,
};

use super::AppState;

pub async fn create_product(
    _claims: Claims,
    State(state): State<AppState>,
    Json(req): Json<CreateProductReq>,
) -> axum::response::Response {
    let name = req.name.clone();
    let product = Product {
        id: 0,
        name: req.name,
        description: req.description,
        price: req.price,
        stock: req.stock,
        category_id: req.category_id,
        active: req.active,
    };
    match state.product_service.create(product).await {
        Ok(id) => {
            tracing::info!(product_id = id, name = %name, "product created");
            (StatusCode::CREATED, Json(CreateProductResp { id })).into_response()
        }
        Err(e) => super::map_error(e),
    }
}

pub async fn get_product(_claims: Claims, State(state): State<AppState>, axum::extract::Path(id): axum::extract::Path<i64>) -> axum::response::Response {
    match state.product_service.get_by_id(id).await {
        Ok(Some(p)) => {
            tracing::info!(product_id = p.id, name = %p.name, active = p.active, "fetched product");
            let resp = ProductResp {
                id: p.id,
                name: p.name,
                description: p.description,
                price: p.price,
                stock: p.stock,
                category_id: p.category_id,
                active: p.active,
            };
            (StatusCode::OK, Json(resp)).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Product not found").into_response(),
        Err(e) => super::map_error(e),
    }
}