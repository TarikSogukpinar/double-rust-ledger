use actix_web::{web, HttpResponse, Result, Scope};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;
use validator::Validate;

use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{Account, ApiResponse, CreateAccountRequest, NewAccount, UpdateAccountRequest};
use crate::schema::accounts::{self, dsl::*};

pub fn config() -> Scope {
    web::scope("/accounts")
        .route("", web::post().to(create_account))
        .route("", web::get().to(get_all_accounts))
        .route("/{id}", web::get().to(get_account))
        .route("/{id}", web::put().to(update_account))
        .route("/{id}", web::delete().to(delete_account))
}

pub async fn create_account(
    pool: web::Data<DbPool>,
    account_data: web::Json<CreateAccountRequest>,
) -> Result<HttpResponse, AppError> {
    account_data
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {:?}", e)))?;

    let mut conn = pool.get()?;
    let account_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let new_account = NewAccount {
        id: account_id.clone(),
        code: account_data.code.clone(),
        name: account_data.name.clone(),
        account_type: account_data.account_type.clone().into(),
        parent_id: account_data.parent_id.clone(),
        is_active: true,
        created_at: now.clone(),
        updated_at: now,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(&mut conn)?;

    let account: Account = accounts::table.find(&account_id).first(&mut conn)?;

    Ok(HttpResponse::Created().json(ApiResponse::success(account)))
}

pub async fn get_all_accounts(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;

    let results: Vec<Account> = accounts::table
        .order(accounts::created_at.desc())
        .load(&mut conn)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(results)))
}

pub async fn get_account(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let account_id = path.into_inner();
    let mut conn = pool.get()?;

    let account: Account = accounts::table.find(&account_id).first(&mut conn)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(account)))
}

pub async fn update_account(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    account_data: web::Json<UpdateAccountRequest>,
) -> Result<HttpResponse, AppError> {
    account_data
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {:?}", e)))?;

    let account_id = path.into_inner();
    let mut conn = pool.get()?;
    let now = Utc::now().to_rfc3339();

    // Build update query dynamically
    let _update_query = diesel::update(accounts::table.find(&account_id));

    if let Some(ref new_code) = account_data.code {
        diesel::update(accounts::table.find(&account_id))
            .set(accounts::code.eq(new_code))
            .execute(&mut conn)?;
    }
    if let Some(ref new_name) = account_data.name {
        diesel::update(accounts::table.find(&account_id))
            .set(accounts::name.eq(new_name))
            .execute(&mut conn)?;
    }
    if let Some(ref new_account_type) = account_data.account_type {
        diesel::update(accounts::table.find(&account_id))
            .set(accounts::account_type.eq(String::from(new_account_type.clone())))
            .execute(&mut conn)?;
    }
    if let Some(ref new_parent_id) = account_data.parent_id {
        diesel::update(accounts::table.find(&account_id))
            .set(accounts::parent_id.eq(new_parent_id))
            .execute(&mut conn)?;
    }
    if let Some(new_is_active) = account_data.is_active {
        diesel::update(accounts::table.find(&account_id))
            .set(accounts::is_active.eq(new_is_active))
            .execute(&mut conn)?;
    }

    // Always update the updated_at field
    diesel::update(accounts::table.find(&account_id))
        .set(accounts::updated_at.eq(now))
        .execute(&mut conn)?;

    let updated_account: Account = accounts::table.find(&account_id).first(&mut conn)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(updated_account)))
}

pub async fn delete_account(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let account_id = path.into_inner();
    let mut conn = pool.get()?;

    let deleted_rows = diesel::delete(accounts::table.find(&account_id)).execute(&mut conn)?;

    if deleted_rows == 0 {
        return Err(AppError::NotFound("Account not found".to_string()));
    }

    Ok(HttpResponse::NoContent().json(ApiResponse::success("Account deleted successfully")))
}
