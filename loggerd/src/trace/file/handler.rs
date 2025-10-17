use super::rotation::RotationConfig;
use super::writer::{TraceMessage, writer_thread};
use crate::trace::{Trace, TraceLevel, handlers::TraceHandler};
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{Sender, channel};
use std::thread::{self, JoinHandle};

/// File trace handler with automatic rotation.
///
/// This handler provides asynchronous file logging with automatic rotation
/// based on file size. It uses a dedicated background thread for all I/O
/// operations to ensure non-blocking performance.
///
/// # Architecture
///
/// - **Dedicated Thread**: Asynchronous, non-blocking writes
/// - **MPSC Channel**: Thread-safe communication
/// - **Rotation**: Size-based file rotation
/// - **Counter**: Atomic count of written logs (AtomicU64)
/// - **Cross-Platform**: Support for Unix and Windows file sharing
///
/// # Performance
///
/// The handler is designed for high-throughput logging scenarios:
/// - Non-blocking log calls (messages are queued)
/// - Batched I/O operations in background thread
/// - Atomic counters for metrics without locks
/// - Efficient file rotation with minimal downtime
///
/// # Examples
///
/// ```no_run
/// use loggerd::trace::file::FileTraceHandler;
/// use loggerd::trace::{Trace, TraceLevel};
/// use std::sync::atomic::Ordering;
///
/// # fn main() -> Result<(), std::io::Error> {
/// // Default configuration (10 MB, 5 backups)
/// let handler = FileTraceHandler::new("app.log")?.start()?;
///
/// // Log some messages
/// handler.log(TraceLevel::Info, "Application started");
/// handler.log(TraceLevel::Warning, "High memory usage detected");
///
/// // Check metrics
/// let count = handler.log_counter().load(Ordering::Relaxed);
/// println!("Logs written: {}", count);
///
/// // Custom configuration
/// let handler = FileTraceHandler::with_rotation("app.log", 5 * 1024 * 1024, 3)?.start()?;
/// handler.log(TraceLevel::Info, "Custom rotation enabled");
/// # Ok(())
/// # }
/// ```
pub struct FileTraceHandler {
    /// Channel sender for communicating with the writer thread
    sender: Option<Sender<TraceMessage>>,
    /// Handle to the background writer thread
    thread_handle: Option<JoinHandle<()>>,
    /// Path to the log file
    file_path: String,
    /// Rotation configuration
    config: RotationConfig,
    /// Shared atomic counter for log metrics
    log_count: Arc<AtomicU64>,
}

impl FileTraceHandler {
    /// Creates a new handler with default configuration (10 MB, 5 backups).
    ///
    /// **Note**: Call `.start()` to begin the writer thread
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the log file
    ///
    /// # Returns
    ///
    /// * `Ok(FileTraceHandler)` - If the file can be accessed
    /// * `Err(std::io::Error)` - If the file cannot be created or accessed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use loggerd::trace::file::FileTraceHandler;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let handler = FileTraceHandler::new("app.log")?;
    /// let handler = handler.start()?; // Start the writer thread
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        Self::with_config(file_path, RotationConfig::default())
    }

    /// Creates a handler with custom rotation parameters.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the log file
    /// * `max_size_bytes` - Maximum file size before rotation (in bytes)
    /// * `max_backups` - Maximum number of backup files to keep
    ///
    /// # Returns
    ///
    /// * `Ok(FileTraceHandler)` - If the file can be accessed
    /// * `Err(std::io::Error)` - If the file cannot be created or accessed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use loggerd::trace::file::FileTraceHandler;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// // 5 MB limit, keep 3 backup files
    /// let handler = FileTraceHandler::with_rotation("app.log", 5_000_000, 3)?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)] // Public API for future use
    pub fn with_rotation(
        file_path: &str,
        max_size_bytes: u64,
        max_backups: usize,
    ) -> Result<Self, std::io::Error> {
        Self::with_config(file_path, RotationConfig::new(max_size_bytes, max_backups))
    }

    /// Creates a handler with a complete configuration.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the log file
    /// * `config` - Complete rotation configuration
    ///
    /// # Returns
    ///
    /// * `Ok(FileTraceHandler)` - If the file can be accessed
    /// * `Err(std::io::Error)` - If the file cannot be created or accessed
    pub fn with_config(file_path: &str, config: RotationConfig) -> Result<Self, std::io::Error> {
        // Check that the file can be created/opened (early validation)
        if File::open(file_path).is_err() {
            File::create_new(file_path)?;
        }

        Ok(Self {
            sender: None,
            thread_handle: None,
            file_path: file_path.to_string(),
            config,
            log_count: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Returns the shared log counter for metrics collection.
    ///
    /// The returned counter is updated atomically each time a log message
    /// is successfully written to the file. This can be used for monitoring
    /// and metrics dashboards.
    ///
    /// # Returns
    ///
    /// Shared atomic counter that tracks the number of log messages written
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use loggerd::trace::file::FileTraceHandler;
    /// use std::sync::atomic::Ordering;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let handler = FileTraceHandler::new("app.log")?.start()?;
    /// let counter = handler.log_counter();
    ///
    /// // Later, check how many logs have been written
    /// let count = counter.load(Ordering::Relaxed);
    /// println!("Total logs written: {}", count);
    /// # Ok(())
    /// # }
    /// ```
    pub fn log_counter(&self) -> Arc<AtomicU64> {
        self.log_count.clone()
    }

    /// Starts the writer thread and returns self for method chaining (Builder pattern).
    ///
    /// This method initializes the background thread responsible for file I/O operations.
    /// After calling this method, the handler is ready to receive log messages.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the writer thread was started successfully
    /// * `Err(std::io::Error)` - If the file cannot be opened for writing
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use loggerd::trace::file::FileTraceHandler;
    /// use loggerd::trace::{Trace, TraceLevel};
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let handler = FileTraceHandler::new("app.log")?.start()?;
    /// handler.log(TraceLevel::Info, "Handler started successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub fn start(mut self) -> Result<Self, std::io::Error> {
        if self.sender.is_some() {
            return Ok(self); // Already started
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = self.file_path.clone();
        let config = self.config.clone();
        let log_count = self.log_count.clone();

        // Dedicated thread for writing with rotation
        let thread_handle = thread::spawn(move || {
            writer_thread(file_path, receiver, config, log_count);
        });

        self.sender = Some(sender);
        self.thread_handle = Some(thread_handle);

        Ok(self)
    }
}

impl Trace for FileTraceHandler {
    fn log(&self, level: TraceLevel, message: &str) {
        if let Some(sender) = &self.sender {
            let formatted = format!("{} - {}\n", level, message);
            // Non-blocking send to writer thread
            let _ = sender.send(TraceMessage::Log(formatted));
        } else {
            eprintln!("Warning: FileTraceHandler not started, call start() first");
        }
    }
}

impl TraceHandler for FileTraceHandler {}

// FileTraceHandler is Send because Sender is Send and JoinHandle is Send
// FileTraceHandler is Sync because we only use a Sender (which implements Sync)
unsafe impl Send for FileTraceHandler {}
unsafe impl Sync for FileTraceHandler {}

impl Drop for FileTraceHandler {
    fn drop(&mut self) {
        // Send shutdown signal if the handler was started
        if let Some(sender) = &self.sender {
            let _ = sender.send(TraceMessage::Shutdown);
        }

        // Wait for the thread to terminate gracefully
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
