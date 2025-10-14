use crate::trace::{Trace, handlers::TraceHandler};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

enum TraceMessage {
    Log(String),
    Shutdown,
}

pub struct FileTraceHanlder {
    sender: Option<Sender<TraceMessage>>,
    thread_handle: Option<JoinHandle<()>>,
    file_path: String,
    max_size_bytes: u64,
    max_backups: usize,
    log_count: Arc<AtomicU64>,
}

impl FileTraceHanlder {
    /// Crée un nouveau handler avec rotation par défaut (10 MB, 5 backups)
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        Self::with_rotation(file_path, 10 * 1024 * 1024, 5) // 10 MB, 5 fichiers de backup
    }

    /// Crée un handler avec paramètres de rotation personnalisés
    pub fn with_rotation(file_path: &str, max_size_bytes: u64, max_backups: usize) -> Result<Self, std::io::Error> {
        // Vérifier que le fichier peut être créé/ouvert (validation précoce)
        let doesnot_exist = File::open(file_path).is_err();
        if doesnot_exist {
            File::create_new(file_path)?;
        }

        Ok(Self {
            sender: None,
            thread_handle: None,
            file_path: file_path.to_string(),
            max_size_bytes,
            max_backups,
            log_count: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Retourne le compteur de logs (pour métriques)
    pub fn log_counter(&self) -> Arc<AtomicU64> {
        self.log_count.clone()
    }

    /// Démarre le thread d'écriture et retourne self pour le chaînage (Builder pattern)
    pub fn start(mut self) -> Result<Self, std::io::Error> {
        if self.sender.is_some() {
            return Ok(self); // Déjà démarré
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = self.file_path.clone();
        let max_size = self.max_size_bytes;
        let max_backups = self.max_backups;
        let log_count = self.log_count.clone();

        // Thread dédié pour l'écriture avec rotation
        let thread_handle = thread::spawn(move || {
            Self::writer_thread(file_path, receiver, max_size, max_backups, log_count);
        });

        self.sender = Some(sender);
        self.thread_handle = Some(thread_handle);

        Ok(self)
    }

    /// Ouvre le fichier en mode append avec partage de lecture
    fn open_log_file(path: &Path) -> std::io::Result<File> {
        #[cfg(unix)]
        {
            // Sur Unix, utiliser les permissions standard
            OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o644) // rw-r--r-- : propriétaire peut lire/écrire, autres peuvent lire
                .open(path)
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            const FILE_SHARE_READ: u32 = 0x00000001;
            const FILE_SHARE_WRITE: u32 = 0x00000002;

            // Sur Windows, autoriser explicitement le partage en lecture
            OpenOptions::new()
                .append(true)
                .create(true)
                .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
                .open(path)
        }

        #[cfg(not(any(unix, windows)))]
        {
            // Fallback pour autres OS
            OpenOptions::new().append(true).create(true).open(path)
        }
    }

    /// Logique du thread d'écriture avec rotation (séparée pour la testabilité)
    fn writer_thread(
        file_path: String,
        receiver: Receiver<TraceMessage>,
        max_size_bytes: u64,
        max_backups: usize,
        log_count: Arc<AtomicU64>,
    ) {
        let path = Path::new(&file_path);

        // Ouvrir en mode append avec partage de lecture explicite
        let mut file = match Self::open_log_file(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return; // Sortie propre sans panic (fichier jamais ouvert, rien à fermer)
            }
        };

        let mut current_size = file.metadata().map(|m| m.len()).unwrap_or(0);

        loop {
            match receiver.recv() {
                Ok(TraceMessage::Log(message)) => {
                    let message_len = message.len() as u64;

                    // Vérifier si rotation nécessaire AVANT d'écrire
                    if current_size + message_len > max_size_bytes {
                        // Flush et fermer le fichier actuel
                        let _ = file.flush();
                        drop(file);

                        // Rotation des fichiers
                        if let Err(e) = Self::rotate_log_files(&file_path, max_backups) {
                            eprintln!("Failed to rotate log files: {}", e);
                        }

                        // Réouvrir un nouveau fichier
                        file = match Self::open_log_file(path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("Failed to reopen log file after rotation: {}", e);
                                return;
                            }
                        };
                        current_size = 0;
                    }

                    // Écrire le message
                    if let Err(e) = file.write_all(message.as_bytes()) {
                        eprintln!("Failed to write log: {}", e);
                    } else {
                        current_size += message_len;
                        log_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    let _ = file.flush();
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

    /// Rotation des fichiers : trace.log -> trace.log.1 -> trace.log.2 -> ...
    fn rotate_log_files(file_path: &str, max_backups: usize) -> std::io::Result<()> {
        use std::fs;
        use chrono::Local;

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
        
        // Renommer trace.log -> trace.log.1.YYYYMMDD_HHMMSS
        fs::rename(file_path, &backup_name)?;
        
        eprintln!("Log rotated: {} (max_size exceeded)", backup_name);
        Ok(())
    }
}

impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) {
        if let Some(sender) = &self.sender {
            let message = format!("{} - {}\n", level, message);
            // Envoi non-bloquant vers le thread d'écriture
            let _ = sender.send(TraceMessage::Log(message));
        } else {
            eprintln!("Warning: FileTraceHandler not started, call start() first");
        }
    }
}

impl TraceHandler for FileTraceHanlder {}

// FileTraceHandler est Send car Sender est Send et JoinHandle est Send
// FileTraceHandler est Sync car on utilise uniquement un Sender (qui implémente Sync)
unsafe impl Send for FileTraceHanlder {}
unsafe impl Sync for FileTraceHanlder {}

impl Drop for FileTraceHanlder {
    fn drop(&mut self) {
        // Envoyer le signal d'arrêt si le handler a été démarré
        if let Some(sender) = &self.sender {
            let _ = sender.send(TraceMessage::Shutdown);
        }

        // Attendre que le thread termine proprement
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
