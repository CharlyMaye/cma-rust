use crate::trace::{Trace, handlers::TraceHandler};

/// A trace handler that prints messages to standard output.
///
/// PrintTraceHandler provides a simple console output mechanism for trace messages.
/// All messages are formatted with their trace level and printed to stdout.
///
/// # Examples
///
/// ```
/// use traces::trace::{PrintTraceHandler, TraceLevel, Trace};
///
/// let handler = PrintTraceHandler::new();
/// handler.log(TraceLevel::Info, "Hello, world!");
/// // Output: [INFO] - Hello, world!
/// ```
pub struct PrintTraceHandler {}

impl PrintTraceHandler {
    /// Creates a new PrintTraceHandler.
    ///
    /// # Returns
    ///
    /// A new PrintTraceHandler ready to output trace messages to the console
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PrintTraceHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Trace for PrintTraceHandler {
    fn log(&self, level: super::TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}

impl TraceHandler for PrintTraceHandler {}
