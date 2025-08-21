use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_address: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:ledger.db".to_string()),
            bind_address: env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }
}
