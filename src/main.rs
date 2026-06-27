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

fn main() {
    println!("Hello, world!");
}
