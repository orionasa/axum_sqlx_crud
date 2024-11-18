mod api;
mod db;
mod error;
mod middleware;
mod models;

use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize database
    let pool = db::create_pool().await;
    db::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    // Create router with middleware
    let app = Router::new()
        .route("/items", get(api::v1::get_items))
        .layer(axum::middleware::from_fn(middleware::pagination_middleware))
        .with_state(pool);

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}