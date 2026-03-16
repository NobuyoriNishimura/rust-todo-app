mod hello;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(crate::hello::say_hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
