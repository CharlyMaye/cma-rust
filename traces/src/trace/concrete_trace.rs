use super::level::TraceLevel;
use super::trace::Trace;


pub struct ConcreteTrace{}
impl ConcreteTrace {
    pub fn new() -> Self {
        Self {}
    }
}
impl Trace for ConcreteTrace {
    fn log(&self, level: TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}