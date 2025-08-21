use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::fmt;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct DatabaseError(pub String);

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database error: {}", self.0)
    }
}

impl Error for DatabaseError {}

pub fn create_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().max_size(15).build(manager)?;

    log::info!("Database pool created successfully");
    Ok(pool)
}

pub fn run_migrations(pool: &DbPool) -> Result<()> {
    let mut connection = pool.get()?;

    log::info!("Running database migrations...");
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| DatabaseError(format!("Migration failed: {}", e)))?;

    log::info!("Database migrations completed successfully");
    Ok(())
}
