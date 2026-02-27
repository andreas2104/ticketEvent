use crate::handlers::user_handler;
use crate::repositories::user_repository::UserRepository;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn user_routes(repo: Arc<UserRepository>) -> Router {
    Router::new()
        .route(
            "/users",
            get(user_handler::get_users).post(user_handler::create_user),
        )
        .route(
            "/users/:id",
            get(user_handler::get_user)
                .put(user_handler::update_user)
                .delete(user_handler::delete_user),
        )
        .with_state(repo)
}
