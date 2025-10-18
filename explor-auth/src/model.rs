use mongodb::Database;
use std::sync::Mutex;

use crate::authentication::AuthService;
use crate::db::MongoConnection;
use crate::documents::DocumentService;

pub struct AppState {
    #[allow(dead_code)]
    pub app_name: String,
    #[allow(dead_code)]
    pub counter: Mutex<i32>,
    pub auth_service: AuthService,
    #[allow(dead_code)]
    pub db: Database,
    pub document_service: DocumentService,
}

impl AppState {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        // Initialiser MongoDB via le module dédié
        let mongo_connection = MongoConnection::new().await?;
        let database = mongo_connection.database();

        // Initialiser le service documents
        let document_service = DocumentService::new(database).await?;

        // Initialiser le service d'authentification
        let auth_service = AuthService::new();

        Ok(AppState {
            app_name: "My Actix-web App".into(),
            counter: Mutex::new(0),
            auth_service,
            db: database.clone(),
            document_service,
        })
    }
}
