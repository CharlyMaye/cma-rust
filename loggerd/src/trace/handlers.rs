use crate::trace::Trace;

/// Trait for handling trace logs.
pub trait TraceHandler: Trace + Send + Sync {}
