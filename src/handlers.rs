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