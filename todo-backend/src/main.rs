mod hello;
mod todo;
use axum::{
    Router,
    routing::{delete, get, post},
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

#[derive(Clone)]
pub struct AppState {
    database_pool: sqlx::mysql::MySqlPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let state = AppState {
        database_pool: pool,
    };

    let app = Router::new()
        .route("/api/hello", get(crate::hello::say_hello))
        .route("/api/todo/add", post(crate::todo::add))
        .route("/api/todo/delete/{id}", delete(crate::todo::delete))
        .route("/api/todo/check", get(crate::todo::check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("The server is ready.");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
