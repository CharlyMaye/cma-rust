use std::sync::Mutex;
use std::collections::HashMap;
use mongodb::{Client, Database};

use crate::authentication::Session;
use crate::documents::data_provider::DocumentDataProvider;
use crate::config::DatabaseConfig;

pub struct AppState {
    pub app_name: String,
    pub counter: Mutex<i32>,
    pub sessions: Mutex<HashMap<String, Session>>,
    pub db: Database,
}

impl AppState {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let config = DatabaseConfig::from_env();
        let client = Client::with_uri_str(&config.uri).await?;
        let database = client.database(&config.database_name);
        
        // Test de la connexion
        client.list_database_names().await?;
        println!("✅ Connected to MongoDB: {}", config.database_name);
        
        Ok(AppState {
            app_name: "My Actix-web App".into(),
            counter: Mutex::new(0),
            sessions: Mutex::new(HashMap::new()),
            db: database,
        })
    }

    pub fn get_document_provider(&self) -> DocumentDataProvider {
        DocumentDataProvider::new(&self.db)
    }
}
