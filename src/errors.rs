use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError()
                .json(crate::models::ApiResponse::<()>::error(msg.clone())),
            AppError::ValidationError(msg) => HttpResponse::BadRequest()
                .json(crate::models::ApiResponse::<()>::error(msg.clone())),
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(crate::models::ApiResponse::<()>::error(msg.clone()))
            }
            AppError::BadRequest(msg) => HttpResponse::BadRequest()
                .json(crate::models::ApiResponse::<()>::error(msg.clone())),
            AppError::InternalServerError(msg) => HttpResponse::InternalServerError()
                .json(crate::models::ApiResponse::<()>::error(msg.clone())),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => AppError::NotFound("Record not found".to_string()),
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> Self {
        AppError::DatabaseError(format!("Connection pool error: {}", error))
    }
}
