use crate::trace::{handlers::TraceHandler, Trace};

pub struct PrintTraceHandler {}
impl PrintTraceHandler {
    pub fn new() -> Self {
        Self {}
    }
}
impl Trace for PrintTraceHandler {
    fn log(&self, level: super::TraceLevel, message: &str) -> () {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}
impl TraceHandler for PrintTraceHandler {
}

// TODO - ajouter un crate pour gérer des fichiers
pub struct FileTraceHanlder {}
impl FileTraceHanlder {
    pub fn new() -> Self {
        Self {}
    }
}
impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) -> () {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}
impl TraceHandler for FileTraceHanlder {
}