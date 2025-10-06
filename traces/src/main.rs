use std::{fmt::Display};

#[derive(Debug)]
pub enum TraceLevel {
    Verbose,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    None,
}
impl Display for TraceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            TraceLevel::Verbose => "VERBOSE",
            TraceLevel::Debug => "DEBUG",
            TraceLevel::Info => "INFO",
            TraceLevel::Warning => "WARNING",
            TraceLevel::Error => "ERROR",
            TraceLevel::Critical => "CRITICAL",
            TraceLevel::None => "NONE",
        };
        write!(f, "[{}]", level_str)
    }
}

pub trait Trace {
    fn log(&self, level: TraceLevel, message: &str) -> ();
}

struct ConcreteTrace{}

impl ConcreteTrace {
    fn new() -> Self {
        Self {}
    }
}
impl Trace for ConcreteTrace {
    fn log(&self, level: TraceLevel, message: &str) {
        let message = format!("{} - {}", level, message);
        println!("{message}");
    }
}


fn test<T: Trace>(trace: T) {
    trace.log(TraceLevel::Info, "Hello World!");
}
fn main() {
    let trace = ConcreteTrace::new();
    
    test(trace);
}
