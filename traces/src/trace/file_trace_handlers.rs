use crate::trace::{Trace, handlers::TraceHandler};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::thread::{self, JoinHandle};

enum TraceMessage {
    Log(String),
    Shutdown,
}

pub struct FileTraceHanlder {
    sender: Sender<TraceMessage>,
    thread_handle: Option<JoinHandle<()>>,
}

impl FileTraceHanlder {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let doesnot_exist = File::open(file_path).is_err();
        if doesnot_exist {
            File::create_new(file_path)?;
        }

        let (sender, receiver) = channel::<TraceMessage>();
        let file_path = file_path.to_string();

        // Thread dédié pour l'écriture
        let thread_handle = thread::spawn(move || {
            let path = Path::new(&file_path);
            
            // Ouvrir avec partage de lecture explicite (gestion d'erreur robuste)
            let mut file = match File::options()
                .append(true)
                .read(false)
                .write(true)
                .create(true)
                .open(path)
            {
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
        });

        Ok(Self { 
            sender,
            thread_handle: Some(thread_handle),
        })
    }
}

impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) {
        let message = format!("{} - {}\n", level, message);
        // Envoi non-bloquant vers le thread d'écriture
        let _ = self.sender.send(TraceMessage::Log(message));
    }
}

impl TraceHandler for FileTraceHanlder {}

impl Drop for FileTraceHanlder {
    fn drop(&mut self) {
        // Envoyer le signal d'arrêt
        let _ = self.sender.send(TraceMessage::Shutdown);
        
        // Attendre que le thread termine proprement
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
