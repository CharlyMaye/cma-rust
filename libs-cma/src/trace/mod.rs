mod concrete_trace;
mod file_trace_handlers;
mod handlers;
mod level;
mod print_trace_handlers;
#[allow(clippy::module_inception)]
mod trace;

use std::io::Error;

pub use concrete_trace::ConcreteTrace;
pub use file_trace_handlers::FileTraceHanlder;
pub use print_trace_handlers::PrintTraceHandler;
pub use trace::HandlerRegister;

pub use handlers::TraceHandler;
pub use level::TraceLevel;
pub use trace::Trace;

/// Creates a preconfigured trace instance with common handlers.
///
/// This convenience function creates a ConcreteTrace with both console and file
/// output handlers pre-registered. The file handler writes to "trace.log" in
/// the current directory.
///
/// # Returns
///
/// * `Ok(impl Trace)` - A configured trace instance ready for use
/// * `Err(Error)` - If the file handler cannot be created or started
///
/// # Examples
///
/// ```no_run
/// use traces::trace::{create_trace, TraceLevel, Trace};
///
/// let trace = create_trace().expect("Failed to create trace");
/// trace.log(TraceLevel::Info, "Application started");
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The trace.log file cannot be created
/// - The file handler fails to start
/// - There are insufficient permissions to write to the current directory
// TODO: Add builder pattern for more flexible configuration
pub fn create_trace() -> Result<impl Trace, Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = FileTraceHanlder::new("trace.log")?.start()?;
    trace.register(print_handler);
    trace.register(file_handler);

    Ok(trace)
}
