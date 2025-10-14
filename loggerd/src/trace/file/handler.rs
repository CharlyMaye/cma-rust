use super::rotation::RotationConfig;
use super::writer::{TraceMessage, writer_thread};
use crate::trace::{Trace, TraceLevel, handlers::TraceHandler};
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{Sender, channel};
use std::thread::{self, JoinHandle};

/// Handler de traces vers fichier avec rotation automatique
///
/// # Architecture
///
/// - **Thread dédié** : Écriture asynchrone non-bloquante
/// - **Canal MPSC** : Communication thread-safe
/// - **Rotation** : Basée sur la taille du fichier
/// - **Compteur** : Nombre de logs écrits (AtomicU64)
///
/// # Exemples
///
/// ```no_run
/// use loggerd::trace::file::FileTraceHandler;
///
/// # fn main() -> Result<(), std::io::Error> {
/// // Configuration par défaut (10 MB, 5 backups)
/// let handler = FileTraceHandler::new("app.log")?.start()?;
///
/// // Configuration personnalisée
/// let handler = FileTraceHandler::with_rotation("app.log", 5 * 1024 * 1024, 3)?.start()?;
/// # Ok(())
/// # }
/// ```
pub struct FileTraceHandler {
    sender: Option<Sender<TraceMessage>>,
    thread_handle: Option<JoinHandle<()>>,
    file_path: String,
    config: RotationConfig,
    log_count: Arc<AtomicU64>,
}

impl FileTraceHandler {
    /// Crée un nouveau handler avec configuration par défaut (10 MB, 5 backups)
    ///
    /// **Note** : Appeler `.start()` pour démarrer le thread d'écriture
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        Self::with_config(file_path, RotationConfig::default())
    }

    /// Crée un handler avec paramètres de rotation personnalisés
    ///
    /// # Arguments
    ///
    /// * `file_path` - Chemin du fichier de log
    /// * `max_size_bytes` - Taille max avant rotation (en bytes)
    /// * `max_backups` - Nombre max de fichiers de backup
    #[allow(dead_code)] // API publique pour usage futur
    pub fn with_rotation(
        file_path: &str,
        max_size_bytes: u64,
        max_backups: usize,
    ) -> Result<Self, std::io::Error> {
        Self::with_config(file_path, RotationConfig::new(max_size_bytes, max_backups))
    }

    /// Crée un handler avec une configuration complète
    pub fn with_config(file_path: &str, config: RotationConfig) -> Result<Self, std::io::Error> {
        // Vérifier que le fichier peut être créé/ouvert (validation précoce)
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

    /// Retourne le compteur de logs partagé (pour métriques)
    pub fn log_counter(&self) -> Arc<AtomicU64> {
        self.log_count.clone()
    }

    /// Démarre le thread d'écriture et retourne self pour le chaînage (Builder pattern)
    ///
    /// # Exemples
    ///
    /// ```no_run
    /// use loggerd::trace::file::FileTraceHandler;
    ///
    /// # fn main() -> Result<(), std::io::Error> {
    /// let handler = FileTraceHandler::new("app.log")?.start()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn start(mut self) -> Result<Self, std::io::Error> {
        if self.sender.is_some() {
            return Ok(self); // Déjà démarré
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = self.file_path.clone();
        let config = self.config.clone();
        let log_count = self.log_count.clone();

        // Thread dédié pour l'écriture avec rotation
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
            // Envoi non-bloquant vers le thread d'écriture
            let _ = sender.send(TraceMessage::Log(formatted));
        } else {
            eprintln!("Warning: FileTraceHandler not started, call start() first");
        }
    }
}

impl TraceHandler for FileTraceHandler {}

// FileTraceHandler est Send car Sender est Send et JoinHandle est Send
// FileTraceHandler est Sync car on utilise uniquement un Sender (qui implémente Sync)
unsafe impl Send for FileTraceHandler {}
unsafe impl Sync for FileTraceHandler {}

impl Drop for FileTraceHandler {
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
