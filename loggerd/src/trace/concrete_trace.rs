use std::sync::{Arc, Mutex};

use super::handlers::TraceHandler;
use super::level::TraceLevel;
use super::trace::{HandlerRegister, Trace};

pub struct ConcreteTrace {
    handlers: Arc<Mutex<Vec<Box<dyn TraceHandler>>>>,
}

impl ConcreteTrace {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl HandlerRegister for ConcreteTrace {
    fn register<T: TraceHandler + 'static>(&self, handler: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }
}

impl Trace for ConcreteTrace {
    fn log(&self, level: TraceLevel, message: &str) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.log(level, message);
        }
    }
}

// ConcreteTrace est Send + Sync car Arc<Mutex<...>> l'est déjà
unsafe impl Send for ConcreteTrace {}
unsafe impl Sync for ConcreteTrace {}
