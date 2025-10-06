use crate::trace::{handlers::TraceHandler, Trace};

pub struct PrintTraceHandler {}
impl PrintTraceHandler {
    pub fn new() -> Self {
        Self {}
    }
}
impl Trace for PrintTraceHandler {
    fn log(&self, level: super::TraceLevel, message: &str) -> () {
        todo!()
    }
}
impl TraceHandler for PrintTraceHandler {
    
}

pub struct FileTraceHanlder {}
impl FileTraceHanlder {
    pub fn new() -> Self {
        Self {}
    }
}
impl Trace for FileTraceHanlder {
    fn log(&self, level: super::TraceLevel, message: &str) -> () {
        todo!()
    }
}
impl TraceHandler for FileTraceHanlder {
    
}