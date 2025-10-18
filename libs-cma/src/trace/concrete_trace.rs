use std::sync::{Arc, Mutex};

use super::handlers::TraceHandler;
use super::level::TraceLevel;
use super::trace::{HandlerRegister, Trace};

// TODO: Consider static lifetime?

/// Concrete implementation of the Trace trait.
///
/// ConcreteTrace manages a collection of trace handlers and forwards
/// all log messages to each registered handler. This allows for flexible
/// logging configurations where messages can be sent to multiple destinations
/// (console, file, network, etc.) simultaneously.
///
/// # Lifetimes
///
/// The `'a` lifetime parameter constrains the handlers to live at least as long
/// as the ConcreteTrace instance, ensuring memory safety when storing handlers.
pub struct ConcreteTrace<'a> {
    /// Thread-safe collection of registered trace handlers
    handlers: Arc<Mutex<Vec<Box<dyn TraceHandler + 'a>>>>,
}

impl<'a> ConcreteTrace<'a> {
    /// Creates a new ConcreteTrace instance with no handlers.
    ///
    /// # Returns
    ///
    /// A new ConcreteTrace ready to accept handler registrations
    ///
    /// # Examples
    ///
    /// ```
    /// use traces::trace::ConcreteTrace;
    ///
    /// let trace = ConcreteTrace::new();
    /// ```
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<'a> Default for ConcreteTrace<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> HandlerRegister<'a> for ConcreteTrace<'a> {
    #[allow(clippy::arc_with_non_send_sync)]
    fn register<T: TraceHandler + 'a>(&self, handler: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }
}

impl<'a> Trace for ConcreteTrace<'a> {
    fn log(&self, level: TraceLevel, message: &str) {
        // Call all registered handlers
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.log(level, message);
        }
    }
}
