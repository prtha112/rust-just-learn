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
