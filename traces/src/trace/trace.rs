use super::handlers::TraceHandler;
use super::level::TraceLevel;

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str);
}

pub trait HandlerRegister<'a> {
    fn register<T: TraceHandler + 'a>(&self, handler: T);
}
