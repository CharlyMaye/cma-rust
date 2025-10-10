use crate::trace::{Trace, handlers::TraceHandler};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver, channel};
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
}

impl FileTraceHanlder {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        // Vérifier que le fichier peut être créé/ouvert (validation précoce)
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

    /// Démarre le thread d'écriture et retourne self pour le chaînage (Builder pattern)
    pub fn start(mut self) -> Result<Self, std::io::Error> {
        if self.sender.is_some() {
            return Ok(self); // Déjà démarré
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = self.file_path.clone();

        // Thread dédié pour l'écriture
        let thread_handle = thread::spawn(move || {
            Self::writer_thread(file_path, receiver);
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
                .mode(0o644)  // rw-r--r-- : propriétaire peut lire/écrire, autres peuvent lire
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
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
        }
    }

    /// Logique du thread d'écriture (séparée pour la testabilité)
    fn writer_thread(file_path: String, receiver: Receiver<TraceMessage>) {
        let path = Path::new(&file_path);
        
        // Ouvrir en mode append avec partage de lecture explicite
        let mut file = match Self::open_log_file(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return; // Sortie propre sans panic (fichier jamais ouvert, rien à fermer)
            }
        };

        loop {
            match receiver.recv() {
                Ok(TraceMessage::Log(message)) => {
                    if let Err(e) = file.write_all(message.as_bytes()) {
                        eprintln!("Failed to write log: {}", e);
                        // Continue même en cas d'erreur d'écriture
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
