use chrono::Local;
use std::fs;
use std::io::Result;

/// Configuration de rotation des logs
#[derive(Debug, Clone)]
pub struct RotationConfig {
    pub max_size_bytes: u64,
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
    #[allow(dead_code)] // API publique pour usage futur
    pub fn new(max_size_bytes: u64, max_backups: usize) -> Self {
        Self {
            max_size_bytes,
            max_backups,
        }
    }
}

/// Effectue la rotation des fichiers de log
///
/// Algorithme :
/// 1. Supprime le plus ancien (file.log.N)
/// 2. Décale tous les fichiers (n -> n+1)
/// 3. Archive le fichier actuel avec timestamp : file.log.1.YYYYMMDD_HHMMSS
///
/// # Exemples
///
/// Avant rotation :
/// ```text
/// loggerd.log (10.1 MB)
/// loggerd.log.1.20251014_120000
/// ```
///
/// Après rotation :
/// ```text
/// loggerd.log (nouveau, vide)
/// loggerd.log.1.20251014_174532
/// loggerd.log.2.20251014_120000
/// ```
pub fn rotate_log_files(file_path: &str, max_backups: usize) -> Result<()> {
    // Supprimer le plus ancien si on a atteint la limite
    if max_backups > 0 {
        let oldest = format!("{}.{}", file_path, max_backups);
        let _ = fs::remove_file(&oldest); // Ignore si n'existe pas
    }

    // Décaler tous les fichiers existants (n -> n+1)
    for i in (1..max_backups).rev() {
        let old_name = format!("{}.{}", file_path, i);
        let new_name = format!("{}.{}", file_path, i + 1);
        let _ = fs::rename(&old_name, &new_name); // Ignore si n'existe pas
    }

    // Archiver le fichier actuel avec timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("{}.1.{}", file_path, timestamp);

    // Renommer file.log -> file.log.1.YYYYMMDD_HHMMSS
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
