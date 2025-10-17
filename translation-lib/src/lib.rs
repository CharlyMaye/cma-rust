/// Simple translation function that maps keys to localized strings
///
/// This is a basic stub implementation. In a complete i18n library, this would:
/// - Load translations from JSON/YAML files
/// - Support multiple locales (en, fr, es, etc.)
/// - Handle pluralization rules
/// - Provide fallback mechanisms
///
/// # Arguments
/// * `key` - The translation key to look up
///
/// # Returns
/// The translated string, or the key itself if no translation is found
///
/// # Examples
/// ```
/// use translation_lib::t;
///
/// assert_eq!(t("hello"), "bonjour");
/// assert_eq!(t("unknown"), "unknown");
/// ```
pub fn t(key: &str) -> String {
    // Very simple stub implementation
    match key {
        "hello" => "bonjour".to_string(),
        _ => key.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_translates() {
        assert_eq!(t("hello"), "bonjour");
    }
}
