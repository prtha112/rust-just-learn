use serde::{Deserialize, Serialize};

use crate::domain::user::User;

#[derive(Serialize, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginReq {
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
    pub active: bool,
    pub greet: String,
}

impl From<User> for UserResp {
    fn from(u: User) -> Self {
        Self {
            greet: u.greet(),
            id: u.id,
            username: u.username,
            active: u.active,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub username: String,
    pub password: String,
}
