use crate::trace::{Trace, handlers::TraceHandler};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::{self, JoinHandle};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

/// Internal message types for communication with the writer thread
enum TraceMessage {
    /// Log a message to the file
    Log(String),
    /// Signal the writer thread to shutdown gracefully
    Shutdown,
}

/// A trace handler that writes messages to a file.
///
/// FileTraceHandler provides asynchronous file logging by using a dedicated
/// background thread for all I/O operations. This prevents blocking the main
/// application thread during log writes.
///
/// # Thread Safety
///
/// The handler uses a background thread and message passing to ensure
/// thread-safe file operations. Multiple threads can log simultaneously
/// without blocking each other.
///
/// # File Sharing
///
/// On supported platforms, the log file is opened with read sharing enabled,
/// allowing other processes to read the file while logging is active.
pub struct FileTraceHanlder {
    /// Channel sender for communicating with the writer thread
    sender: Option<Sender<TraceMessage>>,
    /// Handle to the background writer thread
    thread_handle: Option<JoinHandle<()>>,
    /// Path to the log file
    file_path: String,
}

impl FileTraceHanlder {
    /// Creates a new FileTraceHandler for the specified file path.
    ///
    /// This constructor validates that the file can be created/opened but does not
    /// start the background writer thread. Call `start()` to begin logging.
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
    /// ```
    /// use cma::trace::FileTraceHandler;
    ///
    /// let handler = FileTraceHandler::new("app.log")
    ///     .expect("Failed to create file handler");
    /// ```
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        // Check that the file can be created/opened (early validation)
        let doesnot_exist = File::open(file_path).is_err();
        if doesnot_exist {
            File::create_new(file_path)?;
        }

        Ok(Self {
            sender: None,
            thread_handle: None,
            file_path: file_path.to_string(),
        })
    }

    /// Starts the background writer thread and returns self for method chaining (Builder pattern).
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
    /// ```
    /// use cma::trace::{FileTraceHandler, TraceLevel, Trace};
    ///
    /// let handler = FileTraceHandler::new("app.log")?
    ///     .start()?;
    ///
    /// handler.log(TraceLevel::Info, "Handler started");
    /// ```
    pub fn start(mut self) -> Result<Self, std::io::Error> {
        if self.sender.is_some() {
            return Ok(self); // Already started
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = self.file_path.clone();

        // Dedicated thread for writing
        let thread_handle = thread::spawn(move || {
            Self::writer_thread(file_path, receiver);
        });

        self.sender = Some(sender);
        self.thread_handle = Some(thread_handle);

        Ok(self)
    }

    /// Opens the log file in append mode with read sharing enabled.
    ///
    /// This method configures platform-specific file sharing options to allow
    /// other processes to read the log file while it's being written to.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the log file
    ///
    /// # Returns
    ///
    /// * `Ok(File)` - Successfully opened file handle
    /// * `Err(std::io::Error)` - If the file cannot be opened
    fn open_log_file(path: &Path) -> std::io::Result<File> {
        #[cfg(unix)]
        {
            // On Unix, use standard permissions
            OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o644) // rw-r--r-- : owner can read/write, others can read
                .open(path)
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            const FILE_SHARE_READ: u32 = 0x00000001;
            const FILE_SHARE_WRITE: u32 = 0x00000002;

            // On Windows, explicitly allow read sharing
            OpenOptions::new()
                .append(true)
                .create(true)
                .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
                .open(path)
        }

        #[cfg(not(any(unix, windows)))]
        {
            // Fallback for other OS
            OpenOptions::new().append(true).create(true).open(path)
        }
    }

    /// Background thread logic for writing log messages (separated for testability).
    ///
    /// This function runs in a background thread and handles all file I/O operations.
    /// It processes messages from the main thread until a shutdown signal is received.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the log file
    /// * `receiver` - Channel receiver for trace messages
    fn writer_thread(file_path: String, receiver: Receiver<TraceMessage>) {
        let path = Path::new(&file_path);

        // Open in append mode with explicit read sharing
        let mut file = match Self::open_log_file(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return; // Clean exit without panic (file never opened, nothing to close)
            }
        };

        loop {
            match receiver.recv() {
                Ok(TraceMessage::Log(message)) => {
                    if let Err(e) = file.write_all(message.as_bytes()) {
                        eprintln!("Failed to write log: {}", e);
                        // Continue even on write error
                    }
                    let _ = file.flush();
                }
                Ok(TraceMessage::Shutdown) | Err(_) => {
                    // Final flush and clean shutdown
                    let _ = file.flush();
                    break;
                }
            }
        }
        // File will be automatically closed here (drop)
    }
}

impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) {
        if let Some(sender) = &self.sender {
            let message = format!("{} - {}\n", level, message);
            // Non-blocking send to writer thread
            let _ = sender.send(TraceMessage::Log(message));
        } else {
            eprintln!("Warning: FileTraceHandler not started, call start() first");
        }
    }
}

impl TraceHandler for FileTraceHanlder {}

impl Drop for FileTraceHanlder {
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
