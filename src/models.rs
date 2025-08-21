use crate::schema::*;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: String,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "liability")]
    Liability,
    #[serde(rename = "equity")]
    Equity,
    #[serde(rename = "revenue")]
    Revenue,
    #[serde(rename = "expense")]
    Expense,
}

impl From<String> for AccountType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "asset" => AccountType::Asset,
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => AccountType::Asset,
        }
    }
}

impl From<AccountType> for String {
    fn from(account_type: AccountType) -> Self {
        match account_type {
            AccountType::Asset => "asset".to_string(),
            AccountType::Liability => "liability".to_string(),
            AccountType::Equity => "equity".to_string(),
            AccountType::Revenue => "revenue".to_string(),
            AccountType::Expense => "expense".to_string(),
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateAccountRequest {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateAccountRequest {
    #[validate(length(min = 1, max = 20))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub account_type: Option<AccountType>,
    pub parent_id: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub id: String,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: String,
    pub reference: String,
    pub description: String,
    pub transaction_date: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateTransactionRequest {
    #[validate(length(min = 1, max = 50))]
    pub reference: String,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    pub transaction_date: Option<String>,
    pub entries: Vec<CreateEntryRequest>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub id: String,
    pub reference: String,
    pub description: String,
    pub transaction_date: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = entries)]
pub struct Entry {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub debit_amount: String,
    pub credit_amount: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateEntryRequest {
    pub account_id: String,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    #[validate(length(max = 255))]
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub debit_amount: String,
    pub credit_amount: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AccountBalance {
    pub account_id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub debit_total: Decimal,
    pub credit_total: Decimal,
    pub balance: Decimal,
}

#[derive(Debug, Serialize)]
pub struct TransactionWithEntries {
    pub id: String,
    pub reference: String,
    pub description: String,
    pub transaction_date: String,
    pub created_at: String,
    pub updated_at: String,
    pub entries: Vec<EntryWithAccount>,
}

#[derive(Debug, Serialize)]
pub struct EntryWithAccount {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub account_code: String,
    pub account_name: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceQuery {
    pub account_id: Option<String>,
    pub account_type: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            errors: None,
        }
    }

    pub fn validation_errors(errors: Vec<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some("Validation failed".to_string()),
            errors: Some(errors),
        }
    }
}
