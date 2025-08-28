use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use log::{error, info, warn};
use std::env;
use tokio::signal;
use std::time::Duration;

mod config;
mod database;
mod errors;
mod handlers;
mod middleware;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:ledger.db".to_string());

    info!("Starting Double Entry Ledger API server...");
    info!("Database URL: {}", database_url);

    // Initialize database connection
    let db_pool = database::create_pool(&database_url).expect("Failed to create database pool");

    // Run migrations
    database::run_migrations(&db_pool).expect("Failed to run migrations");

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    info!("Server running at http://{}", bind_address);

    // Create HttpServer
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::PanicRecovery)
            .wrap(middleware::RequestTimeout::new(30)) // 30 second timeout
            .wrap(Logger::default())
            .wrap(actix_cors::Cors::permissive())
            .service(
                web::scope("/api/v1")
                    .service(handlers::accounts::config())
                    .service(handlers::transactions::config())
                    .service(handlers::balance::config()),
            )
            .service(web::resource("/health").route(web::get().to(handlers::health::health_check)))
    })
    .bind(&bind_address)?
    .run();

    // Setup graceful shutdown
    let server_handle = server.handle();
    
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_signal() => {
            info!("Shutdown signal received, starting graceful shutdown...");
            
            // Stop accepting new connections and wait for existing ones to complete
            server_handle.stop(true).await;
            
            info!("Server shutdown completed");
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }
}
