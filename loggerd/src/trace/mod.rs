mod concrete_trace;
pub mod file; // Nouveau module structuré
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

// TODO - add builder
pub fn create_trace() -> Result<(impl Trace + Send + Sync, Arc<AtomicU64>), Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = file::FileTraceHandler::new("loggerd.log")?.start()?;
    let log_counter = file_handler.log_counter();

    trace.register(print_handler);
    trace.register(file_handler);

    Ok((trace, log_counter))
}
