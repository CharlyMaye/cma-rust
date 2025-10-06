use super::level::TraceLevel;
use super::handlers::TraceHandler;

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str) -> ();
}

pub trait HandlerRegister {
    fn register<T: TraceHandler>(&self, handler: T) -> ();
}