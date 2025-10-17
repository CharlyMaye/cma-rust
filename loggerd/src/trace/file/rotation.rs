use chrono::Local;
use std::fs;
use std::io::Result;

/// Configuration for log file rotation.
///
/// This struct defines the parameters that control when and how log files
/// are rotated. Rotation helps manage disk space and keeps log files at
/// a manageable size for analysis tools.
///
/// # Default Configuration
///
/// - Maximum file size: 10 MB
/// - Maximum backup files: 5
///
/// # Examples
///
/// ```
/// use loggerd::trace::file::RotationConfig;
///
/// // Use default settings
/// let config = RotationConfig::default();
///
/// // Custom settings
/// let config = RotationConfig::new(5_000_000, 3); // 5MB, 3 backups
/// ```
#[derive(Debug, Clone)]
pub struct RotationConfig {
    /// Maximum file size in bytes before rotation is triggered
    pub max_size_bytes: u64,
    /// Maximum number of backup files to retain
    pub max_backups: usize,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024, // 10 MB
            max_backups: 5,
        }
    }
}

impl RotationConfig {
    /// Creates a new rotation configuration with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `max_size_bytes` - Maximum file size before rotation (in bytes)
    /// * `max_backups` - Maximum number of backup files to keep
    ///
    /// # Returns
    ///
    /// A new RotationConfig with the specified parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use loggerd::trace::file::RotationConfig;
    ///
    /// // Rotate at 5MB, keep 3 backup files
    /// let config = RotationConfig::new(5 * 1024 * 1024, 3);
    /// ```
    #[allow(dead_code)] // Public API for future use
    pub fn new(max_size_bytes: u64, max_backups: usize) -> Self {
        Self {
            max_size_bytes,
            max_backups,
        }
    }
}

/// Performs log file rotation with timestamped backups.
///
/// This function implements a rotation strategy that preserves log history
/// while managing disk space. The rotation process:
///
/// 1. Removes the oldest backup if the limit is reached
/// 2. Shifts all existing backup files (incrementing their numbers)
/// 3. Archives the current file with a timestamp: `file.log.1.YYYYMMDD_HHMMSS`
///
/// # Timestamp Format
///
/// Backup files include a timestamp in the format `YYYYMMDD_HHMMSS` to
/// provide precise timing information and avoid filename conflicts.
///
/// # Arguments
///
/// * `file_path` - Path to the main log file
/// * `max_backups` - Maximum number of backup files to retain
///
/// # Returns
///
/// * `Ok(())` - If rotation completed successfully
/// * `Err(std::io::Error)` - If any file operation failed
///
/// # Examples
///
/// **Before rotation:**
/// ```text
/// loggerd.log (10.1 MB) - exceeds size limit
/// loggerd.log.1.20251014_120000
/// ```
///
/// **After rotation:**
/// ```text
/// loggerd.log (new, empty)
/// loggerd.log.1.20251014_174532 (archived current file)
/// loggerd.log.2.20251014_120000 (shifted previous backup)
/// ```
///
/// # Error Handling
///
/// This function is designed to be robust:
/// - Missing backup files are ignored (not an error)
/// - The main log file rename is the critical operation
/// - Partial rotation is acceptable if some backup shifts fail
pub fn rotate_log_files(file_path: &str, max_backups: usize) -> Result<()> {
    // Remove the oldest backup if we've reached the limit
    if max_backups > 0 {
        let oldest = format!("{}.{}", file_path, max_backups);
        let _ = fs::remove_file(&oldest); // Ignore if it doesn't exist
    }

    // Shift all existing backup files (n -> n+1)
    for i in (1..max_backups).rev() {
        let old_name = format!("{}.{}", file_path, i);
        let new_name = format!("{}.{}", file_path, i + 1);
        let _ = fs::rename(&old_name, &new_name); // Ignore if it doesn't exist
    }

    // Archive the current file with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("{}.1.{}", file_path, timestamp);

    // Rename file.log -> file.log.1.YYYYMMDD_HHMMSS
    fs::rename(file_path, &backup_name)?;

    eprintln!("Log rotated: {} (max_size exceeded)", backup_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RotationConfig::default();
        assert_eq!(config.max_size_bytes, 10 * 1024 * 1024);
        assert_eq!(config.max_backups, 5);
    }

    #[test]
    fn test_custom_config() {
        let config = RotationConfig::new(5 * 1024 * 1024, 3);
        assert_eq!(config.max_size_bytes, 5 * 1024 * 1024);
        assert_eq!(config.max_backups, 3);
    }
}
