use crate::db::DbPool;
use crate::error::AppError;
use crate::models::{CreateTodo, TodoResponse, UpdateTodo};
use async_trait::async_trait;
use uuid::Uuid;

/// Trait defining todo repository operations
#[async_trait]
pub trait TodoRepository: Send +Sync {
    async fn create(&self, payload: CreateTodo) -> Result<TodoResponse, AppError>;
    async fn list(&self, completed: Option<bool>) -> Result<Vec<TodoResponse>, AppError>;
    async fn get(&self, id: Uuid) -> Result<TodoResponse, AppError>;
    async fn update(&self, id: Uuid, payload: UpdateTodo) -> Result<TodoResponse, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn mark_completed(&self, id: Uuid) -> Result<TodoResponse, AppError>;
}

/// PostgreSQL implementation of TodoRepository
pub struct PostgresTodoRepository {
    pool: DbPool,
}

impl PostgresTodoRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn create(&self, payload: CreateTodo) -> Result<TodoResponse, AppError> {
        let todo = sqlx::query_as!(
            TodoResponse,
            r#"
            INSERT INTO todos (title, description)
            VALUES ($1, $2)
            RETURNING id, title, description, completed as "completed!", created_at as "created_at!", updated_at as "updated_at!"
            "#,
            payload.title,
            payload.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }
    async fn list(&self, completed: Option<bool>) -> Result<Vec<TodoResponse>, AppError> {
        let todos = if let Some(completed) = completed {
            sqlx::query_as!(
                TodoResponse,
                r#"
                SELECT id, title, description, completed as "completed!", created_at as "created_at!", updated_at as "updated_at!"
                FROM todos
                WHERE completed = $1
                ORDER BY created_at DESC
                "#,
                completed
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                TodoResponse,
                r#"
                SELECT id, title, description, completed as "completed!", created_at as "created_at!", updated_at as "updated_at!"
                FROM todos
                ORDER BY created_at DESC
                "#
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(todos)
    }