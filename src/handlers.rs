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