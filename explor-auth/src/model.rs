use std::sync::Mutex;
use std::collections::HashMap;

use crate::authentication::Session;

pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
    pub sessions: Mutex<HashMap<String, Session>>,
}
