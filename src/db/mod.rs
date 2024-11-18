use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

pub type DbPool = Arc<Pool<Postgres>>;

pub async fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    println!("Database connected successfully!");

    Arc::new(pool)
}

pub async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create the items table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )
    "#,
    )
    .execute(pool)
    .await?;

    // Insert test data if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM items")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        for i in 1..=25 {
            sqlx::query!(
                "INSERT INTO items (name) VALUES ($1)",
                format!("Item {}", i)
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
