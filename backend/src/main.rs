use std::net::SocketAddr;
use std::sync::Arc;
use dotenvy::dotenv;
use backend::{create_app, db, repositories};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize database connection pool
    let pool = db::init_db().await;

    // Initialize repositories
    let user_repo = Arc::new(repositories::user_repository::UserRepository::new(pool.clone()));
    let event_repo = Arc::new(repositories::event_repository::EventRepository::new(pool));

    // Build our application with routes
    let app = create_app(user_repo, event_repo);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Backend running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
