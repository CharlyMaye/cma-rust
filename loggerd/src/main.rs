use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new().route("/health", get(|| async { "OK" }));

    // Remplace l'ancien Server::bind(...).serve(...)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("loggerd on http://{}/ (GET /health)", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
