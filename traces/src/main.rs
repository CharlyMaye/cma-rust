use traces::{i18n::create_i18n, trace::{create_trace, Trace, TraceLevel}};

fn test() {
    let trace = create_trace().unwrap();
    trace.log(TraceLevel::Info, "Hello World!");
}
fn test_i18n() {
    let mut state: traces::i18n::I18nState = create_i18n("fr");
    state.load_locale("fr");
}
fn main() {
    test();
    test_i18n();
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_log() {
    }
}
