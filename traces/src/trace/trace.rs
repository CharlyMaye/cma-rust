use super::level::TraceLevel;

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str) -> ();
}