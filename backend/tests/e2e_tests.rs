use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::sync::Arc;
use backend::{create_app, repositories::user_repository::UserRepository, repositories::event_repository::EventRepository};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            email TEXT NOT NULL UNIQUE,
            password TEXT,
            contact TEXT,
            google_id TEXT,
            avatar_url TEXT,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP 
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        date TEXT NOT NULL,
        location TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

#[tokio::test]
async fn test_register_and_login() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let event_repo = Arc::new(EventRepository::new(pool.clone()));
    let app = create_app(user_repo, event_repo);

    // 1. Register a user
    let register_payload = json!({
        "name": "Test User",
        "email": "test@example.com",
        "password": "password123",
        "contact": "123456789"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&register_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Login the user
    let login_payload = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&login_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body.get("token").is_some());
}

#[tokio::test]
async fn test_create_and_get_event() {
    let pool = setup_test_db().await;
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let event_repo = Arc::new(EventRepository::new(pool.clone()));
    let app = create_app(user_repo, event_repo);

    // 1. Create an event
    let event_payload = json!({
        "name": "Test Event",
        "date": "2026-03-01",
        "location": "Test Location"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/event")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&event_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Get events
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/event")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let events: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(events.is_array());
    assert!(events.as_array().unwrap().len() > 0);
}
