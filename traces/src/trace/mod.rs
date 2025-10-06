mod level;
mod trace;

pub use level::TraceLevel;
pub use trace::Trace;
use trace::ConcreteTrace;

// TODO - add builder
pub fn create_trace() -> impl Trace {
    ConcreteTrace::new()
}