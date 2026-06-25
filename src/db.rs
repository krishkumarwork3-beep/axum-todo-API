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