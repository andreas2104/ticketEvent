use axum::{
    routing::post,
    Router,
};
use crate::handlers::auth_handler::{register, login, google_login};
use crate::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub fn auth_routes(state: Arc<UserRepository>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/google-login", post(google_login))
        .with_state(state)
}
