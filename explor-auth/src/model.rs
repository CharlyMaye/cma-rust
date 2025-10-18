use std::sync::Mutex;
use std::collections::HashMap;
use mongodb::Database;

use crate::authentication::Session;
use crate::documents::data_provider::DocumentDataProvider;
use crate::db::MongoConnection;

pub struct AppState {
    pub sessions: Mutex<HashMap<String, Session>>,
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // Initialiser MongoDB via le module dédié
        let mongo_connection = MongoConnection::new().await?;
        
        Ok(AppState {
            sessions: Mutex::new(HashMap::new()),
            db: mongo_connection.database().clone(),
        })
    }

    pub fn get_document_provider(&self) -> DocumentDataProvider {
        DocumentDataProvider::new(&self.db)
    }
}
