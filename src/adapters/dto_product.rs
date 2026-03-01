use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateProductReq {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: i64,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateProductResp {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub struct ProductResp {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category_id: i64,
    pub active: bool,
}