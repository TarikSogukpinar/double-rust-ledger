use actix_web::{web, HttpResponse, Result, Scope};
use chrono::Utc;
use diesel::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;

use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{
    Account, ApiResponse, CreateTransactionRequest, Entry, EntryWithAccount, NewEntry,
    NewTransaction, Transaction, TransactionWithEntries,
};
use crate::schema::{
    accounts::{self, dsl::*},
    entries::{self, dsl::*},
    transactions::{self, dsl::*},
};

pub fn config() -> Scope {
    web::scope("/transactions")
        .route("", web::post().to(create_transaction))
        .route("", web::get().to(get_all_transactions))
        .route("/{id}", web::get().to(get_transaction))
        .route("/{id}", web::delete().to(delete_transaction))
}

pub async fn create_transaction(
    pool: web::Data<DbPool>,
    transaction_data: web::Json<CreateTransactionRequest>,
) -> Result<HttpResponse, AppError> {
    transaction_data
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {:?}", e)))?;

    // Validate double entry - debits must equal credits
    let mut total_debits = Decimal::ZERO;
    let mut total_credits = Decimal::ZERO;

    for entry in &transaction_data.entries {
        if let Some(debit) = entry.debit_amount {
            total_debits += debit;
        }
        if let Some(credit) = entry.credit_amount {
            total_credits += credit;
        }
    }

    if total_debits != total_credits {
        return Err(AppError::ValidationError(
            "Total debits must equal total credits".to_string(),
        ));
    }

    if transaction_data.entries.is_empty() {
        return Err(AppError::ValidationError(
            "Transaction must have at least one entry".to_string(),
        ));
    }

    let mut conn = pool.get()?;

    conn.transaction::<_, AppError, _>(|conn| {
        let new_transaction_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let new_transaction = NewTransaction {
            id: new_transaction_id.clone(),
            reference: transaction_data.reference.clone(),
            description: transaction_data.description.clone(),
            transaction_date: transaction_data
                .transaction_date
                .clone()
                .unwrap_or_else(|| now.clone()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)?;

        // Create entries
        for entry_data in &transaction_data.entries {
            let entry_id = Uuid::new_v4().to_string();

            let new_entry = NewEntry {
                id: entry_id,
                transaction_id: new_transaction_id.clone(),
                account_id: entry_data.account_id.clone(),
                debit_amount: entry_data.debit_amount.unwrap_or(Decimal::ZERO).to_string(),
                credit_amount: entry_data
                    .credit_amount
                    .unwrap_or(Decimal::ZERO)
                    .to_string(),
                description: entry_data.description.clone(),
                created_at: now.clone(),
            };

            diesel::insert_into(entries::table)
                .values(&new_entry)
                .execute(conn)?;
        }

        let transaction: Transaction = transactions::table.find(&new_transaction_id).first(conn)?;

        Ok(transaction)
    })?;

    let created_transaction = get_transaction_with_entries(&mut conn, &transaction_data.reference)?;

    Ok(HttpResponse::Created().json(ApiResponse::success(created_transaction)))
}

pub async fn get_all_transactions(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;

    let results: Vec<Transaction> = transactions::table
        .order(transactions::created_at.desc())
        .load(&mut conn)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(results)))
}

pub async fn get_transaction(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let trans_id = path.into_inner();
    let mut conn = pool.get()?;

    let transaction = get_transaction_with_entries_by_id(&mut conn, &trans_id)?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(transaction)))
}

pub async fn delete_transaction(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let trans_id = path.into_inner();
    let mut conn = pool.get()?;

    let deleted_rows = diesel::delete(transactions::table.filter(transactions::id.eq(&trans_id)))
        .execute(&mut conn)?;

    if deleted_rows == 0 {
        return Err(AppError::NotFound("Transaction not found".to_string()));
    }

    Ok(HttpResponse::NoContent().json(ApiResponse::success("Transaction deleted successfully")))
}

fn get_transaction_with_entries(
    conn: &mut diesel::SqliteConnection,
    ref_id: &str,
) -> Result<TransactionWithEntries, AppError> {
    let transaction: Transaction = transactions::table
        .filter(transactions::reference.eq(ref_id))
        .first(conn)?;

    let transaction_entries: Vec<(Entry, Account)> = entries::table
        .inner_join(accounts::table.on(accounts::id.eq(entries::account_id)))
        .filter(entries::transaction_id.eq(&transaction.id))
        .load(conn)?;

    let entries_with_accounts: Vec<EntryWithAccount> = transaction_entries
        .into_iter()
        .map(|(entry, account)| EntryWithAccount {
            id: entry.id,
            transaction_id: entry.transaction_id,
            account_id: entry.account_id,
            account_code: account.code,
            account_name: account.name,
            debit_amount: entry.debit_amount.parse().unwrap_or(Decimal::ZERO),
            credit_amount: entry.credit_amount.parse().unwrap_or(Decimal::ZERO),
            description: entry.description,
            created_at: entry.created_at,
        })
        .collect();

    Ok(TransactionWithEntries {
        id: transaction.id,
        reference: transaction.reference,
        description: transaction.description,
        transaction_date: transaction.transaction_date,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
        entries: entries_with_accounts,
    })
}

fn get_transaction_with_entries_by_id(
    conn: &mut diesel::SqliteConnection,
    trans_id: &str,
) -> Result<TransactionWithEntries, AppError> {
    let transaction: Transaction = transactions::table.find(trans_id).first(conn)?;

    let transaction_entries: Vec<(Entry, Account)> = entries::table
        .inner_join(accounts::table.on(accounts::id.eq(entries::account_id)))
        .filter(entries::transaction_id.eq(trans_id))
        .load(conn)?;

    let entries_with_accounts: Vec<EntryWithAccount> = transaction_entries
        .into_iter()
        .map(|(entry, account)| EntryWithAccount {
            id: entry.id,
            transaction_id: entry.transaction_id,
            account_id: entry.account_id,
            account_code: account.code,
            account_name: account.name,
            debit_amount: entry.debit_amount.parse().unwrap_or(Decimal::ZERO),
            credit_amount: entry.credit_amount.parse().unwrap_or(Decimal::ZERO),
            description: entry.description,
            created_at: entry.created_at,
        })
        .collect();

    Ok(TransactionWithEntries {
        id: transaction.id,
        reference: transaction.reference,
        description: transaction.description,
        transaction_date: transaction.transaction_date,
        created_at: transaction.created_at,
        updated_at: transaction.updated_at,
        entries: entries_with_accounts,
    })
}
