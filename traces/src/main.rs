use traces::trace::{Trace, TraceLevel, create_trace};


fn test<T: Trace>(trace: T) {
    trace.log(TraceLevel::Info, "Hello World!");
}
fn main() {
    let trace = create_trace();
    test(trace);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
    }
}
