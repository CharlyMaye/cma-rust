mod concrete_trace;
mod concrete_handlers;
mod handlers;
mod level;
mod trace;

pub use level::TraceLevel;
pub use trace::Trace;
use concrete_trace::ConcreteTrace;

use crate::trace::{concrete_handlers::{FileTraceHanlder, PrintTraceHandler}, trace::HandlerRegister};

// TODO - add builder
pub fn create_trace() -> impl Trace {
    let trace = ConcreteTrace::new();

    let printHandler = PrintTraceHandler::new();
    let fileHandler = FileTraceHanlder::new();

    trace.register(printHandler);
    trace.register(fileHandler);

    trace
}
