use mongodb::{Client, Database};
use crate::config::DatabaseConfig;

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
        let collection_names = self.database.list_collection_names().await?;
        
        // Liste des collections requises
        let required_collections = vec!["documents", "users"];
        
        for collection_name in required_collections {
            if !collection_names.contains(&collection_name.to_string()) {
                println!("📝 Creating collection: {}", collection_name);
                self.database.create_collection(collection_name).await?;
            }
        }
        
        Ok(())
    }
    
    pub fn database(&self) -> &Database {
        &self.database
    }
    
    pub fn client(&self) -> &Client {
        &self.client
    }
}
