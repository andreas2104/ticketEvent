use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::repositories::user_repository::UserRepository;
use crate::domain::user::{User, CreateUser};
use std::sync::Arc;

pub async fn create_user(
    State(repo): State<Arc<UserRepository>>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<i64>), (StatusCode, String)> {
    match repo.create(payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn amboarina(
    State(repo)
)

pub async fn get_users(
    State(repo): State<Arc<UserRepository>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match repo.get_all().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_user(
    State(repo): State<Arc<UserRepository>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    match repo.get_by_id(id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_user(
    State(repo): State<Arc<UserRepository>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repo.update(id, payload).await {
        Ok(affected) if affected > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_user(
    State(repo): State<Arc<UserRepository>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repo.delete(id).await {
        Ok(affected) if affected > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
