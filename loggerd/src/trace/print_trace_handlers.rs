use crate::trace::{Trace, handlers::TraceHandler};

/// A simple trace handler that prints log messages to the console.
pub struct PrintTraceHandler {}
impl PrintTraceHandler {
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

// PrintTraceHandler est Send + Sync car sans état partagé
unsafe impl Send for PrintTraceHandler {}
unsafe impl Sync for PrintTraceHandler {}
