use crate::handlers::event_handler;
use crate::repositories::event_repository::EventRepository;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn event_routes(repo: Arc<EventRepository>) -> Router {
    Router::new()
        .route(
            "/event",
            get(event_handler::get_event).post(event_handler::create_event),
        )
        .with_state(repo)
}
