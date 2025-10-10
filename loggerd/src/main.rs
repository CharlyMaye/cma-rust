use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new().route("/health", get(|| async { "OK" }));

    // Remplace l'ancien Server::bind(...).serve(...)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!(
        "loggerd on http://{}/ (GET /health)",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
