# 📊 Double Entry Ledger API

Modern, production-ready double-entry bookkeeping system built with Rust and Actix Web.

## 🚀 Features

- ✅ **Double-Entry Accounting**: Enforced debit/credit balance validation
- 🏦 **Chart of Accounts**: Hierarchical account management (Asset, Liability, Equity, Revenue, Expense)
- 💳 **Transaction Management**: Complete journal entry handling with multiple entries
- ⚖️ **Balance Calculations**: Real-time account balances with proper accounting rules
- 🔒 **Production Ready**: Graceful shutdown, panic recovery, request timeouts
- ⚡ **High Performance**: SQLite with connection pooling and optimized queries
- 🧪 **Well Tested**: Comprehensive unit and integration test suite

## 🛠️ Technology Stack

- **Backend**: Rust + Actix Web 4
- **Database**: SQLite with Diesel ORM
- **Validation**: Serde + Validator
- **Logging**: env_logger
- **Testing**: Built-in Rust testing + Actix Web test utilities

## 📦 Quick Start

### Prerequisites
- Rust 1.70+ installed
- SQLite (for local development)

### Installation

```bash
# Clone the repository
git clone <repo-url>
cd double-rust-ledger

# Run the application
cargo run

# Or with custom configuration
DATABASE_URL="sqlite:custom.db" BIND_ADDRESS="0.0.0.0:3000" cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_transaction_double_entry_flow
```

## 🔌 API Documentation

**Base URL**: `http://localhost:8080`

### Health Check
```http
GET /health
```

### 🏦 Accounts API

#### Create Account
```http
POST /api/v1/accounts
Content-Type: application/json

{
  "code": "1000",
  "name": "Cash Account", 
  "account_type": "asset",
  "parent_id": null
}
```

#### Get All Accounts
```http
GET /api/v1/accounts
```

#### Get Account by ID
```http
GET /api/v1/accounts/{account_id}
```

#### Update Account
```http
PUT /api/v1/accounts/{account_id}
Content-Type: application/json

{
  "name": "Updated Account Name",
  "is_active": true
}
```

#### Delete Account
```http
DELETE /api/v1/accounts/{account_id}
```

### 💳 Transactions API

#### Create Transaction
```http
POST /api/v1/transactions
Content-Type: application/json

{
  "reference": "TXN-001",
  "description": "Cash Sale",
  "transaction_date": "2023-12-01T10:00:00Z",
  "entries": [
    {
      "account_id": "cash-account-id",
      "debit_amount": "1000.00",
      "credit_amount": null,
      "description": "Cash received"
    },
    {
      "account_id": "revenue-account-id", 
      "debit_amount": null,
      "credit_amount": "1000.00",
      "description": "Revenue from sale"
    }
  ]
}
```

#### Get All Transactions
```http
GET /api/v1/transactions
```

#### Get Transaction with Entries
```http
GET /api/v1/transactions/{transaction_id}
```

#### Delete Transaction
```http
DELETE /api/v1/transactions/{transaction_id}
```

### ⚖️ Balance API

#### Get All Balances
```http
GET /api/v1/balance
```

#### Filter by Account Type
```http
GET /api/v1/balance?account_type=asset
```

#### Get Specific Account Balance
```http
GET /api/v1/balance/{account_id}
```

## 📊 Account Types & Balance Rules

| Account Type | Normal Balance | Increases With | Decreases With |
|-------------|---------------|----------------|----------------|
| **Asset** | Debit | Debit | Credit |
| **Liability** | Credit | Credit | Debit |
| **Equity** | Credit | Credit | Debit |
| **Revenue** | Credit | Credit | Debit |
| **Expense** | Debit | Debit | Credit |

## 🧪 Example Transaction Flow

### 1. Create Chart of Accounts
```bash
# Cash Account (Asset)
curl -X POST http://localhost:8080/api/v1/accounts \
  -H "Content-Type: application/json" \
  -d '{"code":"1000","name":"Cash","account_type":"asset"}'

# Revenue Account  
curl -X POST http://localhost:8080/api/v1/accounts \
  -H "Content-Type: application/json" \
  -d '{"code":"4000","name":"Sales Revenue","account_type":"revenue"}'
```

### 2. Record a Sale (Cash received)
```bash
curl -X POST http://localhost:8080/api/v1/transactions \
  -H "Content-Type: application/json" \
  -d '{
    "reference": "SALE-001",
    "description": "Cash sale to customer",
    "entries": [
      {
        "account_id": "cash-account-id",
        "debit_amount": "500.00",
        "credit_amount": null,
        "description": "Cash received from sale"
      },
      {
        "account_id": "revenue-account-id", 
        "debit_amount": null,
        "credit_amount": "500.00",
        "description": "Revenue from sale"
      }
    ]
  }'
```

### 3. Check Balances
```bash
# All account balances
curl http://localhost:8080/api/v1/balance

# Only asset accounts
curl http://localhost:8080/api/v1/balance?account_type=asset
```

## 🔧 Configuration

Environment variables:

- `DATABASE_URL`: SQLite database path (default: `sqlite:ledger.db`)
- `BIND_ADDRESS`: Server bind address (default: `127.0.0.1:8080`)
- `RUST_LOG`: Log level (default: `info`)

## 🏗️ Architecture

```
src/
├── main.rs              # Application entry point + graceful shutdown
├── lib.rs               # Library exports for testing
├── config.rs            # Configuration management
├── database.rs          # Database connection & migrations
├── errors.rs            # Error handling & custom error types
├── middleware.rs        # Recovery & timeout middleware
├── models.rs            # Data models + validation + unit tests
├── schema.rs            # Diesel auto-generated schema
└── handlers/            # API route handlers
    ├── mod.rs
    ├── accounts.rs      # Account CRUD operations
    ├── transactions.rs  # Transaction & entry management
    ├── balance.rs       # Balance calculations & queries
    └── health.rs        # Health check endpoint

migrations/              # Database migrations
tests/                   # Integration tests
```

## 🚦 Error Handling

All API responses follow a consistent format:

**Success Response:**
```json
{
  "success": true,
  "data": { ... },
  "message": null,
  "errors": null
}
```

**Error Response:**
```json
{
  "success": false,
  "data": null,
  "message": "Error description",
  "errors": ["Validation error 1", "Validation error 2"]
}
```

## 📈 Performance Features

- **Connection Pooling**: R2D2 with 15 max connections
- **Request Timeouts**: 30-second timeout protection
- **Panic Recovery**: Graceful error handling for panics
- **Graceful Shutdown**: Clean shutdown on SIGTERM/SIGINT
- **Database Indexing**: Optimized indexes for frequent queries

## 🧪 Testing

The project includes comprehensive tests:

- **Unit Tests**: Model validation, type conversions, business logic
- **Integration Tests**: Full API workflow testing with in-memory database
- **Error Handling Tests**: Validation and error response testing

Key test scenarios:
- Account CRUD operations
- Double-entry transaction validation
- Balance calculation accuracy
- Error handling and validation
- Database constraint enforcement

## 📝 License

MIT License - see LICENSE file for details

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

---

Built with ❤️ in Rust for reliable financial data management.