mod level;
mod concrete_trace;
mod trace;
mod handlers;

pub use level::TraceLevel;
pub use trace::Trace;
use concrete_trace::ConcreteTrace;

// TODO - add builder
pub fn create_trace() -> impl Trace {
    ConcreteTrace::new()
}