use super::level::TraceLevel;

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str) -> ();
}

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