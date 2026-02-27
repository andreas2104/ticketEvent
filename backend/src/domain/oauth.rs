use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}