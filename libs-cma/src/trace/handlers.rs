use crate::trace::Trace;

/// Trait for trace message handlers.
///
/// TraceHandler extends the Trace trait, allowing implementations to both
/// receive and emit trace messages. This design enables handlers to be
/// composed and chained together for complex logging scenarios.
///
/// All types implementing TraceHandler must also implement Trace,
/// providing a consistent interface for message processing.
///
/// # Examples
///
/// ```
/// use traces::trace::{TraceHandler, TraceLevel, Trace};
///
/// struct CustomHandler;
///
/// impl Trace for CustomHandler {
///     fn log(&self, level: TraceLevel, message: &str) {
///         // Custom logging logic
///         println!("{}: {}", level, message);
///     }
/// }
///
/// impl TraceHandler for CustomHandler {}
/// ```
pub trait TraceHandler: Trace {}
