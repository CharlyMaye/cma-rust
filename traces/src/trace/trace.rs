use super::level::TraceLevel;
use super::handlers::TraceHandler;

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str) -> ();
}

pub trait HandlerRegister<'a> {
    fn register<T: TraceHandler + 'a>(&self, handler: T) -> ();
}