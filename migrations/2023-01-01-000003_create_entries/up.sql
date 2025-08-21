CREATE TABLE entries (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    debit_amount TEXT NOT NULL DEFAULT '0',
    credit_amount TEXT NOT NULL DEFAULT '0',
    description TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (transaction_id) REFERENCES transactions (id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
);

CREATE INDEX idx_entries_transaction_id ON entries (transaction_id);
CREATE INDEX idx_entries_account_id ON entries (account_id);