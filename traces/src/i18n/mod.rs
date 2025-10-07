use std::{collections::HashMap, fs::File, io::Read};

use serde_json::from_str;
// TODO - trace ?
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
            default_locale: default_locale.to_string()
        }
    }
}
impl I18nState {
    // TODO - Manage error
    pub fn load_locale(&mut self, lang: &str) {
        let file_name = format!("locales/{lang}.json");
        let mut file_content = String::new();
        
        let mut file_reader = File::open(file_name).unwrap();
        file_reader.read_to_string(&mut file_content).unwrap();
        
        let value: HashMap<String, String> = from_str(&file_content).unwrap();
        self.translations.insert(lang.to_string(), value);
    }
}

pub fn create_i18n(default_locale: &str) -> I18nState {
    I18nState::new(default_locale)
}