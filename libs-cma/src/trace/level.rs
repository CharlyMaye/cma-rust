use std::fmt::Display;

/// Enumeration of trace levels for logging.
///
/// This enum is simple (no heap data) and stored on the stack.
/// We use Copy instead of passing by reference (&TraceLevel) because:
/// - Copy of a simple enum = copy of a few bytes (size of a discriminant)
/// - Reference = copy of a pointer (8 bytes on 64-bit) + memory indirection
/// - Copy is more performant: no dereferencing, direct access to value
/// - Copy is more idiomatic in Rust for primitive/simple types
/// - Simplifies code: no & everywhere, no lifetime management
#[derive(Debug, Copy, Clone)]
pub enum TraceLevel {
    /// Verbose logging - most detailed
    Verbose,
    /// Debug information
    Debug,
    /// General information
    Info,
    /// Warning messages
    Warning,
    /// Error messages
    Error,
    /// Critical errors
    Critical,
    /// No logging
    None,
}

impl Display for TraceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            TraceLevel::Verbose => "VERBOSE",
            TraceLevel::Debug => "DEBUG",
            TraceLevel::Info => "INFO",
            TraceLevel::Warning => "WARNING",
            TraceLevel::Error => "ERROR",
            TraceLevel::Critical => "CRITICAL",
            TraceLevel::None => "NONE",
        };
        write!(f, "[{}]", level_str)
    }
}
