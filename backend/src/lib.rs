pub mod db;
pub mod domain;
pub mod handlers;
pub mod repositories;
pub mod routes;

use std::sync::Arc;
use axum::Router;
use crate::repositories::user_repository::UserRepository;
use crate::routes::user_routes::user_routes;
use crate::repositories::event_repository::EventRepository;
use crate::routes::event_routes::event_routes;

pub fn create_app(user_repo: Arc<UserRepository>, event_repo: Arc<EventRepository>) -> Router {
    Router::new()
        .nest("/api", user_routes(user_repo.clone()))
        .nest("/api", event_routes(event_repo))
        .nest("/api/auth", crate::routes::auth_routes::auth_routes(user_repo.clone()))
        .route("/auth/callback", axum::routing::get(crate::handlers::auth_handler::google_callback).with_state(user_repo))
}
