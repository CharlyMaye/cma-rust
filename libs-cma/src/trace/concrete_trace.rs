use std::sync::{Arc, Mutex};

use super::handlers::TraceHandler;
use super::level::TraceLevel;
use super::trace::{HandlerRegister, Trace};

// TODO - static lifetime?

pub struct ConcreteTrace<'a> {
    handlers: Arc<Mutex<Vec<Box<dyn TraceHandler + 'a>>>>,
}
impl<'a> ConcreteTrace<'a> {
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
impl<'a> HandlerRegister<'a> for ConcreteTrace<'a> {
    #[allow(clippy::arc_with_non_send_sync)]
    fn register<T: TraceHandler + 'a>(&self, handler: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }
}
impl<'a> Trace for ConcreteTrace<'a> {
    fn log(&self, level: TraceLevel, message: &str) {
        // Appeler tous les handlers enregistrés
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.log(level, message);
        }
    }
}
