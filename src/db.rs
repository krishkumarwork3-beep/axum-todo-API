use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Error as SqlxError};

pub type DbPool = Pool<Postgres>;

/// Creates a new database connection pool
pub async fn create_pool(database_url: &str) -> Result<DbPool, SqlxError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Initializes the database (runs migrations if needed)
/// Note: In production, use sqlx-cli for migrations
pub async fn init_db(_pool: &DbPool) -> Result<(), SqlxError> {
    // Migrations should be run via sqlx-cli:
    // sqlx migrate run
    Ok(())
}