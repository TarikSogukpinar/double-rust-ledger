use actix_web::{web, HttpResponse, Result, Scope};
use diesel::prelude::*;
use rust_decimal::Decimal;

use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{Account, AccountBalance, ApiResponse, BalanceQuery, Entry};
use crate::schema::{
    accounts::{self, dsl::*},
    entries::{self, dsl::*}
};

pub fn config() -> Scope {
    web::scope("/balance")
        .route("", web::get().to(get_balances))
        .route("/{account_id}", web::get().to(get_account_balance))
}

pub async fn get_balances(
    pool: web::Data<DbPool>,
    query: web::Query<BalanceQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    
    let mut account_query = accounts::table.into_boxed();
    
    if let Some(ref account_type_filter) = query.account_type {
        account_query = account_query.filter(accounts::account_type.eq(account_type_filter));
    }

    let all_accounts: Vec<Account> = account_query.load(&mut conn)?;
    
    let mut balances = Vec::new();
    
    for account in all_accounts {
        let account_entries: Vec<Entry> = entries::table
            .filter(entries::account_id.eq(&account.id))
            .load(&mut conn)?;
        
        let mut debit_total = Decimal::ZERO;
        let mut credit_total = Decimal::ZERO;
        
        for entry in account_entries {
            debit_total += entry.debit_amount.parse().unwrap_or(Decimal::ZERO);
            credit_total += entry.credit_amount.parse().unwrap_or(Decimal::ZERO);
        }
        
        let balance = match account.account_type.as_str() {
            "asset" | "expense" => debit_total - credit_total,
            "liability" | "equity" | "revenue" => credit_total - debit_total,
            _ => debit_total - credit_total,
        };
        
        balances.push(AccountBalance {
            account_id: account.id,
            account_code: account.code,
            account_name: account.name,
            account_type: account.account_type,
            debit_total,
            credit_total,
            balance,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(balances)))
}

pub async fn get_account_balance(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let acc_id = path.into_inner();
    let mut conn = pool.get()?;
    
    let account: Account = accounts::table
        .find(&acc_id)
        .first(&mut conn)?;
    
    let account_entries: Vec<Entry> = entries::table
        .filter(entries::account_id.eq(&acc_id))
        .load(&mut conn)?;
    
    let mut debit_total = Decimal::ZERO;
    let mut credit_total = Decimal::ZERO;
    
    for entry in account_entries {
        debit_total += entry.debit_amount.parse().unwrap_or(Decimal::ZERO);
        credit_total += entry.credit_amount.parse().unwrap_or(Decimal::ZERO);
    }
    
    let balance = match account.account_type.as_str() {
        "asset" | "expense" => debit_total - credit_total,
        "liability" | "equity" | "revenue" => credit_total - debit_total,
        _ => debit_total - credit_total,
    };
    
    let account_balance = AccountBalance {
        account_id: account.id,
        account_code: account.code,
        account_name: account.name,
        account_type: account.account_type,
        debit_total,
        credit_total,
        balance,
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(account_balance)))
}