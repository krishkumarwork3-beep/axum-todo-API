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

    async fn get(&self, id: Uuid) -> Result<TodoResponse, AppError> {
        let todo = sqlx::query_as!(
            TodoResponse,
            r#"
            SELECT id, title, description, completed as "completed!", created_at as "created_at!", updated_at as "updated_at!"
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Todo with id {} not found", id)))?;

        Ok(todo)
    }

    async fn update(&self, id: Uuid, payload: UpdateTodo) -> Result<TodoResponse, AppError> {
        // Check if todo exists first
        let _existing = sqlx::query!(r#"SELECT id FROM todos WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Todo with id {} not found", id)))?;

        // Build update query dynamically based on provided fields
        let mut query_str = "UPDATE todos SET updated_at = NOW()".to_string();
        let mut param_count = 1;
        let mut title_param: Option<String> = None;
        let mut description_param: Option<String> = None;
        let mut completed_param: Option<bool> = None;

        if payload.title.is_some() {
            query_str.push_str(&format!(", title = ${}", param_count));
            param_count += 1;
            title_param = payload.title;
        }

        if payload.description.is_some() {
            query_str.push_str(&format!(", description = ${}", param_count));
            param_count += 1;
            description_param = payload.description;
        }

        if payload.completed.is_some() {
            query_str.push_str(&format!(", completed = ${}", param_count));
            param_count += 1;
            completed_param = payload.completed;
        }

        query_str.push_str(&format!(
            " WHERE id = ${} RETURNING id, title, description, completed, created_at, updated_at",
            param_count
        ));

        let todo = match (title_param, description_param, completed_param) {
            (Some(title), Some(description), Some(completed)) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(title)
                    .bind(description)
                    .bind(completed)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (Some(title), Some(description), None) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(title)
                    .bind(description)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (Some(title), None, Some(completed)) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(title)
                    .bind(completed)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (Some(title), None, None) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(title)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, Some(description), Some(completed)) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(description)
                    .bind(completed)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, Some(description), None) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(description)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, None, Some(completed)) => {
                sqlx::query_as::<_, TodoResponse>(&query_str)
                    .bind(completed)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, None, None) => {
                sqlx::query_as!(
                    TodoResponse,
                    r#"
                    SELECT id, title, description, completed as "completed!", created_at as "created_at!", updated_at as "updated_at!"
                    FROM todos
                    WHERE id = $1
                    "#,
                    id
                )
                .fetch_one(&self.pool)
                .await?
            }
        };

        Ok(todo)
    }