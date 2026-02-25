use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResp {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub struct SpeakResp {
    pub speak: String,
    pub shout: String,
}

#[derive(Debug, Serialize)]
pub struct UserResp {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub greet: String,
}

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