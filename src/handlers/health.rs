use actix_web::{HttpResponse, Result};
use crate::models::ApiResponse;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("OK".to_string())))
}