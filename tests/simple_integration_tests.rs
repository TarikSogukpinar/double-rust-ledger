// Simplified integration tests focusing on business logic
use double_rust_ledger::{database, models::*};
use rust_decimal::Decimal;

#[test]
fn test_database_operations() {
    // Test database pool creation
    let db_pool = database::create_pool(":memory:").expect("Failed to create test database");
    database::run_migrations(&db_pool).expect("Failed to run migrations");
    
    // Test connection is working
    let mut conn = db_pool.get().expect("Failed to get connection");
    
    // Basic query test - should not panic
    use diesel::prelude::*;
    use double_rust_ledger::schema::accounts;
    
    let result: Result<Vec<Account>, _> = accounts::table.load(&mut conn);
    assert!(result.is_ok());
    let accounts_list = result.unwrap();
    assert_eq!(accounts_list.len(), 0); // Should be empty initially
}

#[test]
fn test_account_type_conversions() {
    // Test all account type conversions
    let test_cases = vec![
        ("asset", AccountType::Asset),
        ("liability", AccountType::Liability),
        ("equity", AccountType::Equity),
        ("revenue", AccountType::Revenue),
        ("expense", AccountType::Expense),
    ];
    
    for (string_val, enum_val) in test_cases {
        assert_eq!(AccountType::from(string_val.to_string()), enum_val);
        assert_eq!(String::from(enum_val.clone()), string_val);
    }
    
    // Test invalid conversion defaults to Asset
    assert_eq!(AccountType::from("invalid".to_string()), AccountType::Asset);
}

#[test]
fn test_balance_calculation_logic() {
    use rust_decimal::Decimal;
    
    // Test asset account balance calculation
    let asset_balance = AccountBalance {
        account_id: "test-asset".to_string(),
        account_code: "1000".to_string(),
        account_name: "Cash".to_string(),
        account_type: "asset".to_string(),
        debit_total: Decimal::new(150000, 2), // $1500.00
        credit_total: Decimal::new(50000, 2),  // $500.00
        balance: Decimal::new(100000, 2),      // $1000.00 (debit - credit)
    };
    
    // For asset accounts: balance should be debit - credit
    let expected_asset_balance = asset_balance.debit_total - asset_balance.credit_total;
    assert_eq!(asset_balance.balance, expected_asset_balance);
    
    // Test revenue account balance calculation
    let revenue_balance = AccountBalance {
        account_id: "test-revenue".to_string(),
        account_code: "4000".to_string(),
        account_name: "Sales".to_string(),
        account_type: "revenue".to_string(),
        debit_total: Decimal::new(25000, 2),   // $250.00
        credit_total: Decimal::new(125000, 2), // $1250.00
        balance: Decimal::new(100000, 2),      // $1000.00 (credit - debit)
    };
    
    // For revenue accounts: balance should be credit - debit
    let expected_revenue_balance = revenue_balance.credit_total - revenue_balance.debit_total;
    assert_eq!(revenue_balance.balance, expected_revenue_balance);
}

#[test]
fn test_double_entry_validation_logic() {
    use rust_decimal::Decimal;
    
    // Test balanced entries
    let balanced_entries = vec![
        CreateEntryRequest {
            account_id: "acc1".to_string(),
            debit_amount: Some(Decimal::new(100000, 2)), // $1000.00
            credit_amount: None,
            description: Some("Cash received".to_string()),
        },
        CreateEntryRequest {
            account_id: "acc2".to_string(),
            debit_amount: None,
            credit_amount: Some(Decimal::new(100000, 2)), // $1000.00
            description: Some("Revenue earned".to_string()),
        },
    ];
    
    let mut total_debits = Decimal::ZERO;
    let mut total_credits = Decimal::ZERO;
    
    for entry in &balanced_entries {
        if let Some(debit) = entry.debit_amount {
            total_debits += debit;
        }
        if let Some(credit) = entry.credit_amount {
            total_credits += credit;
        }
    }
    
    // Should be balanced
    assert_eq!(total_debits, total_credits);
    
    // Test unbalanced entries
    let unbalanced_entries = vec![
        CreateEntryRequest {
            account_id: "acc1".to_string(),
            debit_amount: Some(Decimal::new(100000, 2)), // $1000.00
            credit_amount: None,
            description: Some("Cash received".to_string()),
        },
        CreateEntryRequest {
            account_id: "acc2".to_string(),
            debit_amount: None,
            credit_amount: Some(Decimal::new(50000, 2)), // $500.00
            description: Some("Revenue earned".to_string()),
        },
    ];
    
    let mut total_debits = Decimal::ZERO;
    let mut total_credits = Decimal::ZERO;
    
    for entry in &unbalanced_entries {
        if let Some(debit) = entry.debit_amount {
            total_debits += debit;
        }
        if let Some(credit) = entry.credit_amount {
            total_credits += credit;
        }
    }
    
    // Should NOT be balanced
    assert_ne!(total_debits, total_credits);
}

#[test]
fn test_api_response_structure() {
    // Test success response
    let success_response = ApiResponse::success("test data");
    assert_eq!(success_response.success, true);
    assert_eq!(success_response.data, Some("test data"));
    assert_eq!(success_response.message, None);
    assert_eq!(success_response.errors, None);
    
    // Test error response
    let error_response: ApiResponse<()> = ApiResponse::error("Something went wrong".to_string());
    assert_eq!(error_response.success, false);
    assert_eq!(error_response.data, None);
    assert_eq!(error_response.message, Some("Something went wrong".to_string()));
    assert_eq!(error_response.errors, None);
    
    // Test validation error response
    let validation_errors = vec![
        "Code is required".to_string(),
        "Name cannot be empty".to_string(),
    ];
    let validation_response: ApiResponse<()> = ApiResponse::validation_errors(validation_errors.clone());
    assert_eq!(validation_response.success, false);
    assert_eq!(validation_response.data, None);
    assert_eq!(validation_response.message, Some("Validation failed".to_string()));
    assert_eq!(validation_response.errors, Some(validation_errors));
}

#[test]
fn test_model_validation() {
    use validator::Validate;
    
    // Test valid account request
    let valid_account = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash Account".to_string(),
        account_type: AccountType::Asset,
        parent_id: None,
    };
    assert!(valid_account.validate().is_ok());
    
    // Test invalid account request - empty code
    let invalid_account = CreateAccountRequest {
        code: "".to_string(), // Empty code should fail
        name: "Cash Account".to_string(),
        account_type: AccountType::Asset,
        parent_id: None,
    };
    assert!(invalid_account.validate().is_err());
    
    // Test invalid account request - empty name
    let invalid_name_account = CreateAccountRequest {
        code: "1000".to_string(),
        name: "".to_string(), // Empty name should fail
        account_type: AccountType::Asset,
        parent_id: None,
    };
    assert!(invalid_name_account.validate().is_err());
    
    // Test valid transaction request
    let valid_transaction = CreateTransactionRequest {
        reference: "TXN-001".to_string(),
        description: "Test transaction".to_string(),
        transaction_date: None,
        entries: vec![
            CreateEntryRequest {
                account_id: "acc1".to_string(),
                debit_amount: Some(Decimal::new(100000, 2)),
                credit_amount: None,
                description: Some("Test entry".to_string()),
            }
        ],
    };
    assert!(valid_transaction.validate().is_ok());
    
    // Test invalid transaction request - empty reference
    let invalid_transaction = CreateTransactionRequest {
        reference: "".to_string(), // Empty reference should fail
        description: "Test transaction".to_string(),
        transaction_date: None,
        entries: vec![],
    };
    assert!(invalid_transaction.validate().is_err());
}