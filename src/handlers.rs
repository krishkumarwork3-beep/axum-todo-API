use crate::error::AppError;
use crate::models::{CreateTodo, TodoResponse, UpdateTodo};
use crate::repository::TodoRepository;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

/// Query parameters for listing todos
#[derive(Debug, Deserialize)]
pub struct TodoFilter {
    completed: Option<bool>,
}

/// Create a new todo
pub async fn create_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let todo = repo.create(payload).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

/// List all todos with optional filtering
pub async fn list_todos(
    State(repo): State<Arc<dyn TodoRepository>>,
    Query(filter): Query<TodoFilter>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = repo.list(filter.completed).await?;
    Ok(Json(todos))
}

/// Get a specific todo by ID
pub async fn get_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = repo.get(id).await?;
    Ok(Json(todo))
}

/// Update a todo (partial update)
pub async fn update_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = repo.update(id, payload).await?;
    Ok(Json(todo))
}