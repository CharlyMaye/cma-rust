use crate::trace::{Trace, handlers::TraceHandler};

/// A trace handler that prints log messages to the console.
///
/// PrintTraceHandler provides immediate console output for trace messages,
/// making it ideal for development, debugging, and providing real-time
/// feedback when running the loggerd daemon.
///
/// # Thread Safety
///
/// This handler is thread-safe as it has no shared state and uses
/// the thread-safe `println!` macro for output.
///
/// # Examples
///
/// ```
/// use loggerd::trace::{PrintTraceHandler, TraceLevel, Trace};
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

impl Trace for PrintTraceHandler {
    fn log(&self, level: super::TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}

impl TraceHandler for PrintTraceHandler {}

// PrintTraceHandler is Send + Sync because it has no shared state
unsafe impl Send for PrintTraceHandler {}
unsafe impl Sync for PrintTraceHandler {}
