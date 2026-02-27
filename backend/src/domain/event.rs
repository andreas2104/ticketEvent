use serde::{Deserialize, Serialize};
use ts_rs::TS;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub location: String,
    pub created_at: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateEvent {
    pub name: String,
    pub date: String,
    pub location: String,
}
