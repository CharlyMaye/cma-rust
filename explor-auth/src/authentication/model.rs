use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ============ DTOs (Data Transfer Objects) ============

/// Identifiants de connexion (DTO Request)
///
/// Utilisés pour authentifier un utilisateur.
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Nom d'utilisateur
    #[schema(example = "test")]
    pub user: String,

    /// Mot de passe
    #[schema(example = "dGVzdA==")]
    pub password: String,
}

/// Données de session utilisateur (DTO Response)
///
/// Retournées après une authentification réussie ou lors de la vérification.
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    /// Identifiant de l'utilisateur
    #[schema(example = "test")]
    pub user_id: String,

    /// Date d'expiration de la session
    #[schema(example = "2025-10-19T12:00:00Z")]
    pub expires_at: DateTime<Utc>,
}

/// Données de déconnexion (DTO Response)
#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    /// Indique si une session a été trouvée et supprimée
    #[schema(example = true)]
    pub session_found: bool,
}

// ============ Entité (Domain Model) ============

/// Session stockée côté serveur (entité métier)
///
/// Représente une session utilisateur active en mémoire.
#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// ============ Conversions ============

impl Session {
    /// Crée une nouvelle session avec une durée de validité par défaut (24h)
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
        }
    }

    /// Vérifie si la session est expirée
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Convertit une Session en SessionResponse (DTO)
    pub fn to_response(&self) -> SessionResponse {
        SessionResponse {
            user_id: self.user_id.clone(),
            expires_at: self.expires_at,
        }
    }
}

impl From<LoginRequest> for (String, String) {
    /// Extrait username et password du DTO
    fn from(req: LoginRequest) -> Self {
        (req.user, req.password)
    }
}
