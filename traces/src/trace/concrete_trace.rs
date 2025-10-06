use std::sync::{Arc, Mutex};

use super::level::TraceLevel;
use super::trace::{Trace, HandlerRegister};
use super::handlers::TraceHandler;

// TODO - static lifetime?

pub struct ConcreteTrace<'a> {
    handlers: Arc<Mutex<Vec<Box<dyn TraceHandler + 'a>>>>,

}
impl<'a> ConcreteTrace<'a> {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
impl<'a> HandlerRegister<'a> for ConcreteTrace<'a> {
    fn register<T: TraceHandler + 'a>(&self, handler: T) -> () {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }  
}
impl<'a> Trace for ConcreteTrace<'a> {
    fn log(&self, level: TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");

        // Appeler tous les handlers enregistrés
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.log(level, message.as_str());
        }
    }
}
