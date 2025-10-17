//! File logging module with automatic rotation.
//!
//! This module provides comprehensive file logging capabilities with automatic
//! rotation based on file size. It's designed for high-performance logging
//! daemons that need to manage log files efficiently.
//!
//! # Architecture
//!
//! - `handler.rs` : Public facade (FileTraceHandler)
//! - `writer.rs` : Asynchronous writer thread
//! - `rotation.rs` : File rotation logic
//! - `file_opener.rs` : Cross-platform file opening (Unix/Windows)
//!
//! # Features
//!
//! - **Asynchronous Writing**: Non-blocking log operations using a dedicated thread
//! - **Automatic Rotation**: Size-based rotation with configurable limits
//! - **Cross-Platform**: Supports Unix and Windows file sharing semantics
//! - **Thread-Safe**: Safe for use from multiple threads simultaneously
//! - **Metrics Integration**: Atomic counters for monitoring log activity
//!
//! # Usage
//!
//! ```no_run
//! use loggerd::trace::file::FileTraceHandler;
//! use loggerd::trace::{Trace, TraceLevel};
//!
//! # fn main() -> Result<(), std::io::Error> {
//! // Basic usage with default rotation (10MB, 5 backups)
//! let handler = FileTraceHandler::new("app.log")?.start()?;
//! handler.log(TraceLevel::Info, "Application started");
//!
//! // Custom rotation settings
//! let handler = FileTraceHandler::with_rotation("app.log", 5_000_000, 3)?.start()?;
//! handler.log(TraceLevel::Info, "Custom rotation enabled");
//! # Ok(())
//! # }
//! ```
//!
//! # File Rotation
//!
//! When a log file exceeds the configured size limit, the rotation process:
//! 1. Closes the current log file
//! 2. Renames existing backup files (incrementing their numbers)
//! 3. Archives the current file with a timestamp
//! 4. Creates a new log file for continued logging
//!
//! Example rotation sequence:
//! ```text
//! Before rotation:
//! app.log (10.1 MB) - exceeds limit
//! app.log.1.20231014_120000
//!
//! After rotation:
//! app.log (new, empty)
//! app.log.1.20231014_174532 (archived current)
//! app.log.2.20231014_120000 (previous backup)
//! ```

mod file_opener;
mod handler;
mod rotation;
mod writer;

// Public re-exports
pub use handler::FileTraceHandler;
#[allow(unused_imports)] // Public API for custom config (future use)
pub use rotation::RotationConfig;
