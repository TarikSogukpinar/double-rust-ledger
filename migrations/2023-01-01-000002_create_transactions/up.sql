CREATE TABLE transactions (
    id TEXT PRIMARY KEY,
    reference TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_transactions_reference ON transactions (reference);
CREATE INDEX idx_transactions_date ON transactions (transaction_date);