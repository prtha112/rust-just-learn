use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryReq {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCategoryResp {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub struct CategoryResp {
    pub id: i64,
    pub name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCategoryReq {
    pub name: String,
}
