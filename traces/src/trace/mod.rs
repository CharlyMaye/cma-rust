mod concrete_trace;
mod file_trace_handlers;
mod print_trace_handlers;
mod handlers;
mod level;
mod trace;

use std::io::Error;

use concrete_trace::ConcreteTrace;
use file_trace_handlers::FileTraceHanlder;
use print_trace_handlers::PrintTraceHandler;
use trace::HandlerRegister;

pub use level::TraceLevel;
pub use trace::Trace;

// TODO - add builder
pub fn create_trace() ->  Result<impl Trace,Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = FileTraceHanlder::new("trace.log")?;

    trace.register(print_handler);
    trace.register(file_handler);

    Ok(trace)
}
