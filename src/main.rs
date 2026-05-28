use axum::{response::Response, http::header, Router, routing::get, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "meow meow rust rust xP" }))
        .route("/favicon.ico", get(favicon))

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{port}");

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn favicon() -> Response {
    let bytes = include_bytes!("../assets/favicon.ico");
    Response::builder()
        .header(header::CONTENT_TYPE, "image/x-icon")
        .body(bytes.as_ref().into())
        .unwrap()
}
