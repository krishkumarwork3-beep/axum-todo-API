#![allow(unused)]
mod db;
mod error;
mod handlers;
mod models;
mod repository;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use db::create_pool;
use dotenvy::dotenv;
use repository::PostgresTodoRepository;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_todo=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    // Create database connection pool
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Connected to database");
    // Build our application with routes
    let app = Router::new()
        .route("/todos", post(handlers::create_todo))
        .route("/todos", get(handlers::list_todos))
        .route("/todos/{id}", get(handlers::get_todo))
        .route("/todos/{id}", patch(handlers::update_todo))
        .route("/todos/{id}", delete(handlers::delete_todo))
        .route("/todos/{id}/complete", patch(handlers::mark_completed))