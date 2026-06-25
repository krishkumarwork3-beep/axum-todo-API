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