use std::fs::{File, OpenOptions};
use std::io::Result;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

/// Opens a log file in append mode with cross-platform read sharing.
///
/// This function handles the platform-specific requirements for opening
/// log files with appropriate sharing permissions. It ensures that:
///
/// - The file is opened in append mode for continuous logging
/// - The file is created if it doesn't exist
/// - Other processes can read the file while it's being written
/// - Platform-specific file permissions are properly set
///
/// # Platform Behavior
///
/// ## Unix/Linux
/// - Uses standard POSIX permissions (0o644: rw-r--r--)
/// - Owner can read/write, others can read
/// - Compatible with standard Unix tools (tail, less, etc.)
///
/// ## Windows
/// - Explicitly enables FILE_SHARE_READ and FILE_SHARE_WRITE
/// - Allows other processes to read the file simultaneously
/// - Compatible with Windows text editors and log viewers
///
/// ## Other Platforms
/// - Falls back to basic append + create mode
/// - May have limited sharing capabilities
///
/// # Arguments
///
/// * `path` - Path to the log file to open
///
/// # Returns
///
/// * `Ok(File)` - Successfully opened file handle ready for writing
/// * `Err(std::io::Error)` - If the file cannot be opened or created
///
/// # Examples
///
/// ```no_run
/// use loggerd::trace::file::file_opener::open_log_file;
/// use std::path::Path;
/// use std::io::Write;
///
/// # fn main() -> Result<(), std::io::Error> {
/// let mut file = open_log_file(Path::new("app.log"))?;
/// writeln!(file, "Log message")?;
/// # Ok(())
/// # }
/// ```
pub fn open_log_file(path: &Path) -> Result<File> {
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
        // Fallback for other operating systems
        OpenOptions::new().append(true).create(true).open(path)
    }
}
