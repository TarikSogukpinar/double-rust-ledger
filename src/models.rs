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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_account_type_conversion() {
        // Test From<String> for AccountType
        assert_eq!(AccountType::from("asset".to_string()), AccountType::Asset);
        assert_eq!(AccountType::from("liability".to_string()), AccountType::Liability);
        assert_eq!(AccountType::from("equity".to_string()), AccountType::Equity);
        assert_eq!(AccountType::from("revenue".to_string()), AccountType::Revenue);
        assert_eq!(AccountType::from("expense".to_string()), AccountType::Expense);
        
        // Test invalid type defaults to Asset
        assert_eq!(AccountType::from("invalid".to_string()), AccountType::Asset);
        
        // Test From<AccountType> for String
        assert_eq!(String::from(AccountType::Asset), "asset");
        assert_eq!(String::from(AccountType::Liability), "liability");
        assert_eq!(String::from(AccountType::Equity), "equity");
        assert_eq!(String::from(AccountType::Revenue), "revenue");
        assert_eq!(String::from(AccountType::Expense), "expense");
    }

    #[test]
    fn test_create_account_request_validation() {
        let valid_request = CreateAccountRequest {
            code: "1000".to_string(),
            name: "Cash Account".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
        };
        
        // Should pass validation
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateAccountRequest {
            code: "".to_string(), // Empty code should fail
            name: "Cash Account".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
        };
        
        // Should fail validation
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_create_transaction_request_validation() {
        let valid_entries = vec![
            CreateEntryRequest {
                account_id: "acc1".to_string(),
                debit_amount: Some(Decimal::new(10000, 2)), // 100.00
                credit_amount: None,
                description: Some("Test debit".to_string()),
            },
            CreateEntryRequest {
                account_id: "acc2".to_string(),
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)), // 100.00
                description: Some("Test credit".to_string()),
            },
        ];

        let valid_request = CreateTransactionRequest {
            reference: "TXN-001".to_string(),
            description: "Test transaction".to_string(),
            transaction_date: None,
            entries: valid_entries,
        };
        
        // Should pass validation
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateTransactionRequest {
            reference: "".to_string(), // Empty reference should fail
            description: "Test transaction".to_string(),
            transaction_date: None,
            entries: vec![],
        };
        
        // Should fail validation
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_api_response_builders() {
        let success_response = ApiResponse::success("test data");
        assert!(success_response.success);
        assert_eq!(success_response.data, Some("test data"));
        assert!(success_response.message.is_none());
        assert!(success_response.errors.is_none());

        let error_response: ApiResponse<()> = ApiResponse::error("error message".to_string());
        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.message, Some("error message".to_string()));
        assert!(error_response.errors.is_none());

        let validation_response: ApiResponse<()> = ApiResponse::validation_errors(vec![
            "field1 is required".to_string(),
            "field2 is invalid".to_string(),
        ]);
        assert!(!validation_response.success);
        assert!(validation_response.data.is_none());
        assert_eq!(validation_response.message, Some("Validation failed".to_string()));
        assert_eq!(validation_response.errors, Some(vec![
            "field1 is required".to_string(),
            "field2 is invalid".to_string(),
        ]));
    }

    #[test]
    fn test_account_balance_calculation() {
        use rust_decimal::Decimal;
        
        let balance = AccountBalance {
            account_id: "test-id".to_string(),
            account_code: "1000".to_string(),
            account_name: "Test Account".to_string(),
            account_type: "asset".to_string(),
            debit_total: Decimal::new(15000, 2), // 150.00
            credit_total: Decimal::new(5000, 2),  // 50.00
            balance: Decimal::new(10000, 2),      // 100.00
        };

        // For asset accounts: balance = debits - credits
        let expected_balance = balance.debit_total - balance.credit_total;
        assert_eq!(balance.balance, expected_balance);
    }
}
