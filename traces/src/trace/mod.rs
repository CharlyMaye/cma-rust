mod concrete_trace;
mod concrete_handlers;
mod handlers;
mod level;
mod trace;

use std::io::Error;

pub use level::TraceLevel;
pub use trace::Trace;
use concrete_trace::ConcreteTrace;

use crate::trace::{concrete_handlers::{FileTraceHanlder, PrintTraceHandler}, trace::HandlerRegister};

// TODO - add builder
pub fn create_trace() ->  Result<impl Trace,Error> {
    let trace = ConcreteTrace::new();

    let print_handler = PrintTraceHandler::new();
    let file_handler = FileTraceHanlder::new("trace.log")?;

    trace.register(print_handler);
    trace.register(file_handler);

    Ok(trace)
}
