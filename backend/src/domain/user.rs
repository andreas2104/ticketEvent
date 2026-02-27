use serde::{Serialize, Deserialize};
use ts_rs::TS;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[ts(skip)]
    pub password: Option<String>,
    pub contact: Option<String>,
    pub google_id: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub created_at: Option<String>,
}

// Structure pour la création d'un utilisateur (sans l'ID)
#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub contact: Option<String>,
    pub role: Option<String>,
}