use super::file_opener::open_log_file;
use super::rotation::{RotationConfig, rotate_log_files};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::Receiver;

/// Messages envoyés au thread d'écriture
pub enum TraceMessage {
    Log(String),
    Shutdown,
}

/// Thread d'écriture dédié avec gestion de la rotation
///
/// Ce thread :
/// - Reçoit les messages via un canal MPSC
/// - Écrit dans le fichier de manière séquentielle
/// - Surveille la taille du fichier
/// - Déclenche la rotation si nécessaire
/// - Maintient un compteur atomique de logs
pub fn writer_thread(
    file_path: String,
    receiver: Receiver<TraceMessage>,
    config: RotationConfig,
    log_count: Arc<AtomicU64>,
) {
    let path = Path::new(&file_path);

    // Ouvrir en mode append avec partage de lecture explicite
    let mut file = match open_log_file(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open log file '{}': {}", file_path, e);
            return; // Sortie propre sans panic
        }
    };

    let mut current_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    loop {
        match receiver.recv() {
            Ok(TraceMessage::Log(message)) => {
                let message_len = message.len() as u64;

                // Vérifier si rotation nécessaire et tenter la rotation
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
                        // Continue avec le fichier actuel même si rotation échoue
                    });
                }

                // Écrire le message
                match write_message(&mut file, &message, message_len, &log_count) {
                    Ok(()) => current_size += message_len,
                    Err(e) => eprintln!("Failed to write log: {}", e),
                }
            }
            Ok(TraceMessage::Shutdown) | Err(_) => {
                // Flush final et fermeture propre
                let _ = file.flush();
                break;
            }
        }
    }
    // Le fichier sera automatiquement fermé ici (drop)
}

/// Vérifie si une rotation est nécessaire
#[inline]
fn should_rotate(current_size: u64, message_len: u64, max_size: u64) -> bool {
    current_size + message_len > max_size
}

/// Effectue la rotation : flush, close, rotate, reopen
fn perform_rotation(
    file: &mut std::fs::File,
    current_size: &mut u64,
    path: &Path,
    file_path: &str,
    max_backups: usize,
) -> std::io::Result<()> {
    // Flush et fermer le fichier actuel
    file.flush()?;
    drop(std::mem::replace(
        file,
        open_log_file(Path::new("/dev/null")).unwrap(),
    ));

    // Rotation des fichiers
    rotate_log_files(file_path, max_backups)?;

    // Réouvrir un nouveau fichier
    *file = open_log_file(path)?;
    *current_size = 0;

    Ok(())
}

/// Écrit un message dans le fichier et incrémente le compteur
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
