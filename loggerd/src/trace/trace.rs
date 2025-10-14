use super::handlers::TraceHandler;
use super::level::TraceLevel;

/// Trait for logging traces with different levels and handlers.
pub trait Trace {
    /// Logs a message with the specified trace level.
    fn log(&self, level: TraceLevel, message: &str);
}

/// Trait for registering trace handlers.
pub trait HandlerRegister {
    /// Registers a new trace handler.
    fn register<T: TraceHandler + 'static>(&self, handler: T);
}
