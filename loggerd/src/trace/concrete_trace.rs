use std::sync::{Arc, Mutex};

use super::handlers::TraceHandler;
use super::level::TraceLevel;
use super::trace::{HandlerRegister, Trace};

/// Concrete implementation of the Trace trait for the loggerd daemon.
///
/// ConcreteTrace manages a collection of trace handlers and forwards
/// all log messages to each registered handler. This allows for flexible
/// logging configurations where messages can be sent to multiple destinations
/// (console, file, network, etc.) simultaneously.
///
/// # Thread Safety
///
/// This implementation is fully thread-safe, using Arc<Mutex<>> to protect
/// the handler collection. Multiple threads can log simultaneously and
/// register new handlers without data races.
///
/// # Usage
///
/// ```
/// use loggerd::trace::{ConcreteTrace, HandlerRegister, PrintTraceHandler, TraceLevel, Trace};
///
/// let trace = ConcreteTrace::new();
/// let handler = PrintTraceHandler::new();
/// trace.register(handler);
/// trace.log(TraceLevel::Info, "Hello, world!");
/// ```
pub struct ConcreteTrace {
    /// Thread-safe collection of registered trace handlers
    handlers: Arc<Mutex<Vec<Box<dyn TraceHandler>>>>,
}

impl ConcreteTrace {
    /// Creates a new ConcreteTrace instance with no handlers.
    ///
    /// # Returns
    ///
    /// A new ConcreteTrace ready to accept handler registrations
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for ConcreteTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlerRegister for ConcreteTrace {
    fn register<T: TraceHandler + 'static>(&self, handler: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }
}

impl Trace for ConcreteTrace {
    fn log(&self, level: TraceLevel, message: &str) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.log(level, message);
        }
    }
}

// ConcreteTrace is Send + Sync because Arc<Mutex<...>> is already Send + Sync
unsafe impl Send for ConcreteTrace {}
unsafe impl Sync for ConcreteTrace {}
