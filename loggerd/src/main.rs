use axum::{Json, Router, extract::State, routing::get};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};

mod trace;
use trace::{Trace, TraceLevel};

/// Shared state for application metrics.
///
/// Maintains counters and timing information for the HTTP API endpoints
/// and integrates with the trace system for logging.
#[derive(Clone)]
struct AppState {
    /// Application metrics counters
    metrics: Arc<MetricsState>,
    /// Shared trace system for logging
    trace: Arc<dyn Trace + Send + Sync>,
}

/// Internal metrics state with atomic counters.
///
/// Thread-safe metrics collection using atomic operations to avoid
/// blocking during concurrent access from multiple request handlers.
struct MetricsState {
    /// Total number of HTTP requests processed
    requests: AtomicU64,
    /// Total number of log messages written (shared with file handler)
    log_count: Arc<AtomicU64>,
    /// Application start time for uptime calculation
    start: Instant,
}

/// Main entry point for the loggerd daemon.
///
/// Initializes the trace system with console and file handlers (with rotation),
/// sets up HTTP API endpoints for health checks and metrics, and runs the server
/// with graceful shutdown support.
///
/// # HTTP Endpoints
///
/// - `GET /health` - Health check endpoint (returns "OK")
/// - `GET /metrics` - JSON metrics including request count, log count, and uptime
///
/// # Graceful Shutdown
///
/// The daemon listens for SIGTERM and SIGHUP signals and shuts down gracefully,
/// ensuring all pending logs are written and resources are cleaned up.
#[tokio::main]
async fn main() {
    // Initialize trace system (console + file with rotation)
    let (trace_system, log_count) =
        trace::create_trace().expect("Failed to initialize trace system");
    let trace_arc: Arc<dyn Trace + Send + Sync> = Arc::new(trace_system);

    trace_arc.log(TraceLevel::Info, "Initializing loggerd daemon...");

    // Shared state for metrics
    let state = AppState {
        metrics: Arc::new(MetricsState {
            requests: AtomicU64::new(0),
            log_count,
            start: Instant::now(),
        }),
        trace: trace_arc.clone(),
    };

    // Configure routes
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

    // Server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state.trace.clone()))
        .await
        .unwrap();

    state
        .trace
        .log(TraceLevel::Info, "loggerd shutdown complete");
}

/// HTTP handler for the health check endpoint.
///
/// Returns a simple "OK" response to indicate the service is running.
/// This endpoint can be used by load balancers and monitoring systems
/// to verify service availability.
///
/// # Returns
///
/// Static string "OK"
async fn health_handler() -> &'static str {
    "OK"
}

/// HTTP handler for the metrics endpoint.
///
/// Returns JSON-formatted metrics including:
/// - Total HTTP requests processed
/// - Total log messages written to files
/// - Service uptime in seconds
/// - Current service status
///
/// This endpoint increments the request counter each time it's called.
///
/// # Returns
///
/// JSON object with current metrics
async fn metrics_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    // Increment request counter
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

/// Handles graceful shutdown on SIGTERM and SIGHUP signals.
///
/// This function sets up signal handlers for SIGTERM and SIGHUP, which are
/// commonly used by process managers and container orchestrators to request
/// graceful shutdown. When either signal is received, the function logs the
/// event and returns, allowing the main server loop to shut down cleanly.
///
/// # Arguments
///
/// * `trace` - Shared trace instance for logging shutdown events
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
