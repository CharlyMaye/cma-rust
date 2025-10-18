use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub uri: String,
    pub database_name: String,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        
        Self {
            database: DatabaseConfig::from_env(),
            server: ServerConfig::from_env(),
            cors: CorsConfig::from_env(),
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let database_name = env::var("MONGODB_DATABASE")
            .unwrap_or_else(|_| "mongodb_rust".to_string());
        
        Self {
            uri,
            database_name,
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "localhost".to_string());
        
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        
        Self { host, port }
    }
}

impl CorsConfig {
    pub fn from_env() -> Self {
        let allowed_origin = env::var("CORS_ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:4200".to_string());
        
        Self { allowed_origin }
    }
}