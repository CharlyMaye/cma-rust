mod rx;
use rx::test_rx;
use traces::{
    i18n::create_i18n,
    trace::{Trace, TraceLevel, create_trace},
};

fn test() {
    let trace = create_trace().unwrap();
    trace.log(TraceLevel::Info, "Hello World!");
}
fn test_i18n() {
    let mut state: traces::i18n::I18nState = create_i18n("fr");
    let _ = state.load_locale("fr");
}
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
