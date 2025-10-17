use super::file_opener::open_log_file;
use super::rotation::{RotationConfig, rotate_log_files};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Receiver;

/// Messages sent to the writer thread.
///
/// This enum defines the communication protocol between the main thread
/// and the background writer thread.
pub enum TraceMessage {
    /// Log a message to the file
    Log(String),
    /// Signal the writer thread to shutdown gracefully
    Shutdown,
}

/// Dedicated writer thread with rotation management.
///
/// This function runs in a background thread and handles all file I/O operations
/// for the FileTraceHandler. It provides:
///
/// - Sequential message processing via MPSC channel
/// - File size monitoring for automatic rotation
/// - Atomic counter updates for metrics
/// - Graceful error handling and recovery
///
/// # Architecture
///
/// The writer thread maintains a simple loop:
/// 1. Receive messages from the main thread
/// 2. Check if rotation is needed before writing
/// 3. Perform rotation if necessary
/// 4. Write the message to the file
/// 5. Update the atomic counter
/// 6. Repeat until shutdown signal
///
/// # Arguments
///
/// * `file_path` - Path to the log file
/// * `receiver` - MPSC receiver for trace messages
/// * `config` - Rotation configuration
/// * `log_count` - Shared atomic counter for metrics
///
/// # Error Handling
///
/// The writer thread is designed to be resilient:
/// - File open errors are logged but don't crash the thread
/// - Write errors are logged and the thread continues
/// - Rotation errors are logged but don't prevent continued logging
/// - The thread always attempts graceful shutdown
pub fn writer_thread(
    file_path: String,
    receiver: Receiver<TraceMessage>,
    config: RotationConfig,
    log_count: Arc<AtomicU64>,
) {
    let path = Path::new(&file_path);

    // Open in append mode with explicit read sharing
    let mut file = match open_log_file(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open log file '{}': {}", file_path, e);
            return; // Clean exit without panic
        }
    };

    let mut current_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    loop {
        match receiver.recv() {
            Ok(TraceMessage::Log(message)) => {
                let message_len = message.len() as u64;

                // Check if rotation is needed and attempt rotation
                if should_rotate(current_size, message_len, config.max_size_bytes) {
                    perform_rotation(
                        &mut file,
                        &mut current_size,
                        path,
                        &file_path,
                        config.max_backups,
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("Rotation failed, continuing with current file: {}", e);
                        // Continue with current file even if rotation fails
                    });
                }

                // Write the message
                match write_message(&mut file, &message, message_len, &log_count) {
                    Ok(()) => current_size += message_len,
                    Err(e) => eprintln!("Failed to write log: {}", e),
                }
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

/// Checks if rotation is needed based on current and incoming message size.
///
/// # Arguments
///
/// * `current_size` - Current file size in bytes
/// * `message_len` - Size of the incoming message in bytes
/// * `max_size` - Maximum allowed file size before rotation
///
/// # Returns
///
/// `true` if the file should be rotated, `false` otherwise
#[inline]
fn should_rotate(current_size: u64, message_len: u64, max_size: u64) -> bool {
    current_size + message_len > max_size
}

/// Performs file rotation: flush, close, rotate, reopen.
///
/// This function handles the complete rotation process:
/// 1. Flushes and closes the current file
/// 2. Calls the rotation logic to move files
/// 3. Opens a new file for continued logging
/// 4. Resets the size counter
///
/// # Arguments
///
/// * `file` - Mutable reference to the current file handle
/// * `current_size` - Mutable reference to the current size counter
/// * `path` - Path to the log file
/// * `file_path` - String path for rotation operations
/// * `max_backups` - Maximum number of backup files to keep
///
/// # Returns
///
/// * `Ok(())` - If rotation completed successfully
/// * `Err(std::io::Error)` - If any step of rotation failed
fn perform_rotation(
    file: &mut std::fs::File,
    current_size: &mut u64,
    path: &Path,
    file_path: &str,
    max_backups: usize,
) -> std::io::Result<()> {
    // Flush and close the current file
    file.flush()?;
    drop(std::mem::replace(
        file,
        open_log_file(Path::new("/dev/null")).unwrap(),
    ));

    // Rotate files
    rotate_log_files(file_path, max_backups)?;

    // Reopen a new file
    *file = open_log_file(path)?;
    *current_size = 0;

    Ok(())
}

/// Writes a message to the file and increments the atomic counter.
///
/// This function performs the actual file write operation and updates
/// the shared metrics counter atomically.
///
/// # Arguments
///
/// * `file` - Mutable reference to the file handle
/// * `message` - The message string to write
/// * `_message_len` - Length of the message (unused, for future optimization)
/// * `log_count` - Shared atomic counter for metrics
///
/// # Returns
///
/// * `Ok(())` - If the write and flush completed successfully
/// * `Err(std::io::Error)` - If the write or flush operation failed
#[inline]
fn write_message(
    file: &mut std::fs::File,
    message: &str,
    _message_len: u64,
    log_count: &Arc<AtomicU64>,
) -> std::io::Result<()> {
    file.write_all(message.as_bytes())?;
    file.flush()?;
    log_count.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_rotate() {
        assert!(!should_rotate(100, 50, 200));
        assert!(should_rotate(100, 101, 200));
        assert!(should_rotate(200, 1, 200));
    }
}
