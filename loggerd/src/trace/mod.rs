mod concrete_trace;
mod file_trace_handlers;
mod handlers;
mod level;
mod print_trace_handlers;
#[allow(clippy::module_inception)]
mod trace;

use std::io::Error;

use concrete_trace::ConcreteTrace;
use file_trace_handlers::FileTraceHanlder;
use print_trace_handlers::PrintTraceHandler;
use trace::HandlerRegister;

pub use level::TraceLevel;
pub use trace::Trace;

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

// TODO - add builder
pub fn create_trace() -> Result<(impl Trace + Send + Sync, Arc<AtomicU64>), Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = FileTraceHanlder::new("loggerd.log")?.start()?;
    let log_counter = file_handler.log_counter();
    
    trace.register(print_handler);
    trace.register(file_handler);

    Ok((trace, log_counter))
}
