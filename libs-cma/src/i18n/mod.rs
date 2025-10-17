use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, Read},
};

use serde_json::from_str;
use std::error::Error;

/// Errors that can occur during i18n operations.
#[derive(Debug)]
pub enum I18nError {
    /// File not found error
    NoFile(io::Error),
    /// I/O operation error
    IoError(io::Error),
    /// JSON deserialization error
    JSONError(serde_json::Error),
}

impl Error for I18nError {}

impl fmt::Display for I18nError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoFile(e) => write!(f, "I/O error: {e}"),
            Self::IoError(e) => write!(f, "I/O error: {e}"),
            Self::JSONError(e) => write!(f, "JSON Deserialisation error: {e}"),
        }
    }
}

impl From<io::Error> for I18nError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

/// Internationalization state manager.
/// 
/// Manages translations for multiple locales and provides methods to load
/// locale-specific translation files. Translations are stored as key-value
/// pairs in JSON format.
/// 
/// # Examples
/// 
/// ```
/// use cma::i18n::{I18nState, create_i18n};
/// 
/// let mut i18n = create_i18n("en");
/// i18n.load_locale("fr").expect("Failed to load French locale");
/// ```
// TODO: Add trace integration for debugging locale loading
#[allow(dead_code)]
pub struct I18nState {
    /// Translations stored as locale -> (key -> translation) mapping
    translations: HashMap<String, HashMap<String, String>>,
    /// Currently active locale
    current_locale: String,
    /// Default fallback locale
    default_locale: String,
}

impl I18nState {
    /// Creates a new I18nState with the specified default locale.
    /// 
    /// # Arguments
    /// 
    /// * `default_locale` - The default locale to use as fallback
    /// 
    /// # Returns
    /// 
    /// A new I18nState instance
    pub fn new(default_locale: &str) -> Self {
        Self {
            translations: HashMap::new(),
            current_locale: default_locale.to_string(),
            default_locale: default_locale.to_string(),
        }
    }
}

impl I18nState {
    /// Loads translations for the specified locale from a JSON file.
    /// 
    /// The file should be located in the `locales/` directory and named
    /// `{lang}.json`. The JSON should contain a flat object with
    /// translation keys and their corresponding values.
    /// 
    /// # Arguments
    /// 
    /// * `lang` - The locale code (e.g., "en", "fr", "de")
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If the locale was loaded successfully
    /// * `Err(I18nError)` - If an error occurred during loading
    /// 
    /// # Errors
    /// 
    /// * `I18nError::NoFile` - If the locale file doesn't exist
    /// * `I18nError::IoError` - If there was an I/O error reading the file
    /// * `I18nError::JSONError` - If the JSON couldn't be parsed
    pub fn load_locale(&mut self, lang: &str) -> Result<(), I18nError> {
        let file_name = format!("locales/{lang}.json");
        let mut file_content = String::new();

        let mut file_reader = match File::open(file_name) {
            Ok(content) => content,
            Err(e) => return Err(I18nError::NoFile(e)),
        };

        match file_reader.read_to_string(&mut file_content) {
            Ok(_) => (),
            Err(e) => return Err(I18nError::IoError(e)),
        };

        let value: HashMap<String, String> = match from_str(&file_content) {
            Ok(value) => value,
            Err(e) => return Err(I18nError::JSONError(e)),
        };
        self.translations.insert(lang.to_string(), value);

        Ok(())
    }
}

/// Creates a new I18n state with the specified default locale.
/// 
/// This is a convenience function that creates an I18nState instance.
/// 
/// # Arguments
/// 
/// * `default_locale` - The default locale to use
/// 
/// # Returns
/// 
/// A new I18nState instance
/// 
/// # Examples
/// 
/// ```
/// use cma::i18n::create_i18n;
/// 
/// let i18n = create_i18n("en");
/// ```
pub fn create_i18n(default_locale: &str) -> I18nState {
    I18nState::new(default_locale)
}
