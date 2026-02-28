mod adapters;
mod domain;
mod infra;
mod usecases;

use std::sync::Arc;

use adapters::restapi::{router, AppState};
use infra::user_repository::PostgresUserRepository;
use infra::category_repository::PostgresCategoryRepository;
use usecases::user_service::UserService;
use usecases::category_service::CategoryService;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize OpenTelemetry — keep providers alive until shutdown
    let otel = infra::telemetry::init_telemetry()
        .expect("Failed to initialize OpenTelemetry");

    tracing::info!("OpenTelemetry initialized");

    let url_db = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let max_connection = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTIONS must be set")
        .parse::<u32>()
        .expect("MAX_CONNECTIONS must be a number");
    let listen_port = env::var("LISTEN_PORT")
        .expect("LISTEN_PORT must be set");
    let address = format!("0.0.0.0:{}", listen_port);

    let pool = PgPoolOptions::new()
        .max_connections(max_connection)
        .connect(&url_db)
        .await
        .expect("Failed to connect to database");

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let user_service = UserService::new(user_repo);

    let category_repo = Arc::new(PostgresCategoryRepository::new(pool));
    let category_service = CategoryService::new(category_repo);

    let state = AppState { user_service, category_service };
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();
    tracing::info!("Listening on http://{}", address);

    // Run server until Ctrl+C, then gracefully flush OTel before exit
    tokio::select! {
        result = axum::serve(listener, app) => {
            if let Err(e) = result {
                tracing::error!("Server error: {e}");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutting down — flushing telemetry...");
        }
    }

    // Flush and shutdown OTel providers BEFORE process exits
    otel.shutdown();
}
