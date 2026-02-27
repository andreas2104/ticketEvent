use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::domain::user::User;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GoogleLoginRequest {
    pub token: String,
}
