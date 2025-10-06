use std::collections::HashMap;

use super::level::TraceLevel;
use super::trace::{Trace, HandlerRegister};
use super::handlers::TraceHandler;

pub struct ConcreteTrace{
    // TODO - HashMap

}
impl ConcreteTrace {
    pub fn new() -> Self {
        Self {}
    }
}
impl HandlerRegister for ConcreteTrace {
    fn register<T: TraceHandler>(&self, handler: T) -> () {
        
    }    
}
impl Trace for ConcreteTrace {
    fn log(&self, level: TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}
