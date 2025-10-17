mod rx;
use rx::test_rx;
use traces::{
    i18n::create_i18n,
    trace::{Trace, TraceLevel, create_trace},
};

/// Test function for the trace system.
///
/// Creates a trace instance and logs an info message to verify
/// the trace system is working properly.
fn test() {
    let trace = create_trace().unwrap();
    trace.log(TraceLevel::Info, "Hello World!");
}

/// Test function for the internationalization system.
///
/// Creates an i18n state with French locale and loads
/// the French translations to verify the i18n system.
fn test_i18n() {
    let mut state: traces::i18n::I18nState = create_i18n("fr");
    let _ = state.load_locale("fr");
}

/// Main entry point for the application.
///
/// Runs test functions for the trace system, i18n system,
/// and reactive (rx) system to verify all components are working.
fn main() {
    test();
    test_i18n();
    test_rx();
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_log() {}
}
