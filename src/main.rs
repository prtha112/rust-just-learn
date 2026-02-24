mod adapters;
mod domain;
mod infra;
mod usecases;

use std::sync::Arc;

use adapters::http::{router, AppState};
use infra::postgres::PostgresUserRepository;
use usecases::user_service::UserService;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url_db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db)
        .await
        .expect("Failed to connect to database");

    let repo = Arc::new(PostgresUserRepository::new(pool));
    let user_service = UserService::new(repo);

    let state = AppState { user_service };
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3555").await.unwrap();
    println!("Listening on http://0.0.0.0:3555");
    axum::serve(listener, app).await.unwrap();
}