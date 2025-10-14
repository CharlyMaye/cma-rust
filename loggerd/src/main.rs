use axum::{extract::State, routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};

mod trace;
use trace::{Trace, TraceLevel};

/// État partagé pour les métriques
#[derive(Clone)]
struct AppState {
    metrics: Arc<MetricsState>,
    trace: Arc<dyn Trace + Send + Sync>,
}

struct MetricsState {
    requests: AtomicU64,
    log_count: Arc<AtomicU64>,
    start: Instant,
}

#[tokio::main]
async fn main() {
    // Initialisation du système de traces (console + fichier avec rotation)
    let (trace_system, log_count) = trace::create_trace().expect("Failed to initialize trace system");
    let trace_arc: Arc<dyn Trace + Send + Sync> = Arc::new(trace_system);
    
    trace_arc.log(TraceLevel::Info, "Initializing loggerd daemon...");

    // État partagé pour les métriques
    let state = AppState {
        metrics: Arc::new(MetricsState {
            requests: AtomicU64::new(0),
            log_count,
            start: Instant::now(),
        }),
        trace: trace_arc.clone(),
    };

    // Configuration des routes
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone());

    // Bind TCP
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    let msg = format!(
        "loggerd started on http://{}/ (GET /health, /metrics)",
        listener.local_addr().unwrap()
    );
    state.trace.log(TraceLevel::Info, &msg);

    // Serveur avec graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state.trace.clone()))
        .await
        .unwrap();

    state.trace.log(TraceLevel::Info, "loggerd shutdown complete");
}

/// Handler pour /health
async fn health_handler() -> &'static str {
    "OK"
}

/// Handler pour /metrics
async fn metrics_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    // Incrémenter le compteur de requêtes
    let requests = state.metrics.requests.fetch_add(1, Ordering::Relaxed) + 1;
    let logs = state.metrics.log_count.load(Ordering::Relaxed);
    let uptime = state.metrics.start.elapsed().as_secs();

    Json(json!({
        "requests": requests,
        "log_count": logs,
        "uptime_seconds": uptime,
        "status": "running"
    }))
}

/// Gestion du shutdown gracieux sur SIGTERM et SIGHUP
async fn shutdown_signal(trace: Arc<dyn Trace + Send + Sync>) {
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM handler");
    let mut sighup = signal(SignalKind::hangup()).expect("Failed to setup SIGHUP handler");

    tokio::select! {
        _ = sigterm.recv() => {
            trace.log(TraceLevel::Warning, "Received SIGTERM, shutting down gracefully...");
        }
        _ = sighup.recv() => {
            trace.log(TraceLevel::Warning, "Received SIGHUP, shutting down gracefully...");
        }
    }
}
