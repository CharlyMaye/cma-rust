use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

/// Identifiants de connexion
/// 
/// Utilisés pour authentifier un utilisateur.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginCredentials {
    /// Nom d'utilisateur
    #[schema(example = "test")]
    pub user: String,
    
    /// Mot de passe
    #[schema(example = "dGVzdA==")]
    pub password: String,
}

/// Données de session utilisateur
/// 
/// Retournées après une authentification réussie.
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionData {
    /// Identifiant de l'utilisateur
    #[schema(example = "test")]
    pub user_id: String,
    
    /// Date d'expiration de la session
    #[schema(example = "2025-10-19T12:00:00Z")]
    pub expires_at: DateTime<Utc>,
}

/// Session stockée côté serveur
#[derive(Clone)]
pub struct Session {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    /// Convertit une Session en SessionData pour l'API
    pub fn to_data(&self) -> SessionData {
        SessionData {
            user_id: self.user_id.clone(),
            expires_at: self.expires_at,
        }
    }
}