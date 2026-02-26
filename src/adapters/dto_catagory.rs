use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateCatagoryReq {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCatagoryResp {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub struct CatagoryResp {
    pub id: i64,
    pub name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCatagoryReq {
    pub name: String,
}