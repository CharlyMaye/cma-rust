use crate::config::DatabaseConfig;
use mongodb::{Client, Database};

pub struct MongoConnection {
    client: Client,
    database: Database,
}

impl MongoConnection {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let config = DatabaseConfig::from_env();
        let client = Client::with_uri_str(&config.uri).await?;
        let database = client.database(&config.database_name);

        let connection = Self { client, database };

        // Vérifier et initialiser la database
        connection.initialize().await?;

        Ok(connection)
    }

    async fn initialize(&self) -> Result<(), mongodb::error::Error> {
        // Vérifier la connexion
        self.client.list_database_names().await?;

        // Vérifier si la database existe
        let db_names = self.client.list_database_names().await?;
        let db_exists = db_names.contains(&self.database.name().to_string());

        if !db_exists {
            println!("📦 Creating new database: {}", self.database.name());
        } else {
            println!("✅ Database found: {}", self.database.name());
        }

        // Initialiser les collections nécessaires
        self.initialize_collections().await?;

        println!("✅ Connected to MongoDB: {}", self.database.name());

        Ok(())
    }

    async fn initialize_collections(&self) -> Result<(), mongodb::error::Error> {
        // Initialiser la collection users (pour l'authentification)
        let collection_names = self.database.list_collection_names().await?;
        if !collection_names.contains(&"users".to_string()) {
            self.create_users_collection().await?;
        }

        Ok(())
    }

    async fn create_users_collection(&self) -> Result<(), mongodb::error::Error> {
        println!("📝 Creating collection: users");
        self.database.create_collection("users").await?;

        // Ici, on pourrait ajouter des index pour la collection users si nécessaire

        Ok(())
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    #[allow(dead_code)]
    pub fn client(&self) -> &Client {
        &self.client
    }
}
