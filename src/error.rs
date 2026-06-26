use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    // Generic errors
    ServerError,
    BadRequest,
    Unauthorized,
    PermissionDenied,

    // Todo specific errors
    TodoNotFound,
    TodoValidationError,
    TodoAlreadyCompleted,

    // Auth related (keep for future use)
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    UserNotAuthenticated,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::BadRequest => "Bad request".to_string(),
            ErrorMessage::Unauthorized => "Unauthorized".to_string(),
            ErrorMessage::PermissionDenied => {
                "You are not allowed to perform this action".to_string()
            }
            ErrorMessage::TodoNotFound => "Todo not found".to_string(),
            ErrorMessage::TodoValidationError => "Validation error".to_string(),
            ErrorMessage::TodoAlreadyCompleted => "Todo is already completed".to_string(),