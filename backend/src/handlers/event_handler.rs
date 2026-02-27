use crate::domain::event::{Event, CreateEvent};
use crate::repositories::event_repository::EventRepository;
use axum::{Json, extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn create_event(
    State(repo): State<Arc<EventRepository>>,
    Json(payload): Json<CreateEvent>,
) -> Result<(StatusCode, Json<i64>), (StatusCode, String)> {
    match repo.create(payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_event(
    State(repo): State<Arc<EventRepository>>,
) -> Result<Json<Vec<Event>>, (StatusCode, String)> {
    match repo.find_all().await {
        Ok(events) => Ok(Json(events)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
