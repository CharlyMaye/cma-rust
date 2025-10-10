use crate::trace::{Trace, handlers::TraceHandler};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::thread;

pub struct FileTraceHanlder {
    sender: Sender<String>,
}

impl FileTraceHanlder {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let doesnot_exist = File::open(file_path).is_err();
        if doesnot_exist {
            File::create_new(file_path)?;
        }

        let (sender, receiver) = channel::<String>();
        let file_path = file_path.to_string();

        // Thread dédié pour l'écriture
        thread::spawn(move || {
            let path = Path::new(&file_path);
            let mut file = File::options()
                .append(true)
                .open(path)
                .expect("Failed to open log file");

            while let Ok(message) = receiver.recv() {
                if let Err(e) = file.write_all(message.as_bytes()) {
                    eprintln!("Failed to write log: {}", e);
                }
                let _ = file.flush();
            }
        });

        Ok(Self { sender })
    }
}

impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) {
        let message = format!("{} - {}\n", level, message);
        // Envoi non-bloquant vers le thread d'écriture
        let _ = self.sender.send(message);
    }
}

impl TraceHandler for FileTraceHanlder {}
