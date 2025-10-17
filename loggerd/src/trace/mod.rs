mod concrete_trace;
pub mod file; // New structured module
mod handlers;
mod level;
mod print_trace_handlers;
#[allow(clippy::module_inception)]
mod trace;

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use concrete_trace::ConcreteTrace;
use print_trace_handlers::PrintTraceHandler;
use trace::HandlerRegister;

pub use level::TraceLevel;
pub use trace::Trace;

/// Creates a preconfigured trace system for the loggerd daemon.
///
/// This function sets up a complete logging system with:
/// - Console output handler for immediate feedback
/// - File handler with automatic rotation (writes to "loggerd.log")
/// - Shared atomic counter for metrics tracking
///
/// The file handler is configured with default rotation settings:
/// - Maximum file size: 10 MB
/// - Maximum backup files: 5
/// - Rotation includes timestamps for backup files
///
/// # Returns
///
/// * `Ok((impl Trace + Send + Sync, Arc<AtomicU64>))` - Configured trace system and log counter
/// * `Err(Error)` - If the file handler cannot be created or started
///
/// # Examples
///
/// ```
/// use loggerd::trace::{create_trace, TraceLevel};
///
/// let (trace, counter) = create_trace().expect("Failed to create trace system");
/// trace.log(TraceLevel::Info, "Daemon started");
/// println!("Logs written: {}", counter.load(std::sync::atomic::Ordering::Relaxed));
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The loggerd.log file cannot be created
/// - The file handler thread fails to start
/// - There are insufficient permissions to write to the current directory
// TODO: Add builder pattern for more flexible configuration
pub fn create_trace() -> Result<(impl Trace + Send + Sync, Arc<AtomicU64>), Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = file::FileTraceHandler::new("loggerd.log")?.start()?;
    let log_counter = file_handler.log_counter();

    trace.register(print_handler);
    trace.register(file_handler);

    Ok((trace, log_counter))
}
