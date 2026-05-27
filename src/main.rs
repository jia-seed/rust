use axum::{extract::Path, Router, routing::get, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "meow meow rust rust xP" }))
        .route("/hello/:visitor", get(greet_visitor));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn greet_visitor(Path(visitor): Path<String>) -> String {
    format!("Hello, {visitor}!")
}