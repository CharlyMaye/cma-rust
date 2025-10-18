use super::handlers::TraceHandler;
use super::level::TraceLevel;

/// Trait for types that can emit trace messages.
///
/// This trait defines the core logging interface that all trace implementations
/// must provide. It allows emitting messages at different severity levels.
pub trait Trace {
    /// Log a message at the specified trace level.
    ///
    /// # Arguments
    ///
    /// * `level` - The severity level of this message
    /// * `message` - The message content to log
    ///
    /// # Examples
    ///
    /// ```
    /// use traces::trace::{Trace, TraceLevel, create_trace};
    ///
    /// let trace = create_trace().unwrap();
    /// trace.log(TraceLevel::Info, "Application started");
    /// trace.log(TraceLevel::Error, "Failed to connect to database");
    /// ```
    fn log(&self, level: TraceLevel, message: &str);
}

/// Trait for types that can register trace handlers.
///
/// This trait allows trace implementations to accept and register
/// multiple handlers that will process trace messages. Each handler
/// can implement different output mechanisms (console, file, network, etc.).
pub trait HandlerRegister<'a> {
    /// Register a new trace handler.
    ///
    /// The handler will receive all trace messages emitted by this trace instance.
    /// Multiple handlers can be registered to send trace messages to different
    /// destinations simultaneously.
    ///
    /// # Arguments
    ///
    /// * `handler` - The trace handler to register
    ///
    /// # Examples
    ///
    /// ```
    /// use traces::trace::{ConcreteTrace, HandlerRegister, PrintTraceHandler};
    ///
    /// let trace = ConcreteTrace::new();
    /// let handler = PrintTraceHandler::new();
    /// trace.register(handler);
    /// ```
    fn register<T: TraceHandler + 'a>(&self, handler: T);
}
