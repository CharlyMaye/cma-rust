use crate::trace::{handlers::TraceHandler, Trace};
use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::path::Path;

// TODO - ajouter un crate pour gérer des fichiers
pub struct FileTraceHanlder {
    file_path: String
}
impl FileTraceHanlder {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        // Est ce que le fichier existe?
        let doesnot_exist = File::open(file_path).is_err();
        if doesnot_exist {
            // TODO - Manage error !
            File::create_new(file_path)?;
        }
        // Si non, on crée le fichier
        Ok(Self {
            file_path: file_path.to_string()
        })
    }
}
impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) -> () {
        let message = format!("{} - {}\n", level, message);
        let path = Path::new(self.file_path.as_str());
        let display = path.display();
        // TODO - Manage error !
        // TODO - manage in async/thread 
        let mut file = match File::options()
            .write(true)
            .append(true)
            .open(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        file.seek(SeekFrom::End(0)).unwrap();

        // TODO - Manage error !
        match file.write_all(message.as_str().as_bytes()){
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        };
    }
}
impl TraceHandler for FileTraceHanlder {
}
