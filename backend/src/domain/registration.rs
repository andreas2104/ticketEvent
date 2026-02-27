use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Registration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub contact: String
}

