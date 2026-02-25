mod adapters;
mod domain;
mod infra;
mod usecases;

use std::sync::Arc;

use adapters::http::{router, AppState};
use infra::user_repository::PostgresUserRepository;
use infra::catagory_repository::PostgresCatagoryRepository;
use usecases::user_service::UserService;
use usecases::catagory_service::CatagoryService;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
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

    let catagory_repo = Arc::new(PostgresCatagoryRepository::new(pool));
    let catagory_service = CatagoryService::new(catagory_repo);

    let state = AppState { user_service, catagory_service };
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();
    println!("Listening on http://{}", address);
    axum::serve(listener, app)
        .await
        .unwrap();
}