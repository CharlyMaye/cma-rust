use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{self, Read},
};

use serde_json::from_str;
use std::error::Error;

#[derive(Debug)]
pub enum I18nError {
    NoFile(io::Error),
    IoError(io::Error),
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

// TODO - trace ?
#[allow(dead_code)]
pub struct I18nState {
    translations: HashMap<String, HashMap<String, String>>,
    current_locale: String,
    default_locale: String,
}
impl I18nState {
    pub fn new(default_locale: &str) -> Self {
        Self {
            translations: HashMap::new(),
            current_locale: default_locale.to_string(),
            default_locale: default_locale.to_string(),
        }
    }
}
impl I18nState {
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

pub fn create_i18n(default_locale: &str) -> I18nState {
    I18nState::new(default_locale)
}
