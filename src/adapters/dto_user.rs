use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
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
    pub username: String,
    pub password: String,
    pub active: bool,
    pub greet: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub username: String,
    pub password: String,
}