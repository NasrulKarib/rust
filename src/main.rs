use std::net::SocketAddr;

use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_backend=info,tower_http=info".into()),
        )
        .init();

    let app = Router::new()
        .route("/api/hello", get(hello))
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8169);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello world from Rust backend!".to_string(),
    })
}
