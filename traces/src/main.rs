use traces::trace::{Trace, TraceLevel, ConcreteTrace};


fn test<T: Trace>(trace: T) {
    trace.log(TraceLevel::Info, "Hello World!");
}
fn main() {
    let trace = ConcreteTrace::new();
    
    test(trace);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
    }
}
