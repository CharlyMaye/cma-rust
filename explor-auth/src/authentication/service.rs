use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use super::model::{LoginRequest, LogoutResponse, Session, SessionResponse};
use super::utils;

/// Erreurs possibles lors des opérations d'authentification
#[derive(Debug)]
pub enum AuthError {
    /// Les identifiants fournis sont invalides
    InvalidCredentials,
    /// Aucune session n'a été trouvée
    SessionNotFound,
    /// La session a expiré
    SessionExpired,
    /// Erreur lors de l'accès au stockage des sessions
    StorageError,
}

/// Service d'authentification
///
/// Gère la logique métier de l'authentification :
/// - Validation des identifiants
/// - Création et gestion des sessions
/// - Vérification des sessions actives
pub struct AuthService {
    sessions: Mutex<HashMap<String, Session>>,
}

impl AuthService {
    /// Crée une nouvelle instance du service d'authentification
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Authentifie un utilisateur et crée une session
    ///
    /// # Arguments
    /// * `request` - Les identifiants de connexion
    ///
    /// # Returns
    /// * `Ok((session_id, SessionResponse))` - L'ID de session et les données de session
    /// * `Err(AuthError)` - Erreur si les identifiants sont invalides
    pub fn login(&self, request: LoginRequest) -> Result<(String, SessionResponse), AuthError> {
        let (username, password) = request.into();

        // Valider les identifiants
        if !utils::validate_credentials(&username, &password) {
            return Err(AuthError::InvalidCredentials);
        }

        // Créer une nouvelle session
        let session_id = Uuid::new_v4().to_string();
        let session = Session::new(username);
        let session_response = session.to_response();

        // Stocker la session
        let mut sessions = self.sessions.lock().map_err(|_| AuthError::StorageError)?;

        sessions.insert(session_id.clone(), session);

        Ok((session_id, session_response))
    }

    /// Vérifie si une session est valide
    ///
    /// # Arguments
    /// * `session_id` - L'identifiant de session à vérifier
    ///
    /// # Returns
    /// * `Ok(SessionResponse)` - Les données de session si valide
    /// * `Err(AuthError)` - Erreur si la session est invalide ou expirée
    pub fn verify_session(&self, session_id: &str) -> Result<SessionResponse, AuthError> {
        let sessions = self.sessions.lock().map_err(|_| AuthError::StorageError)?;

        match sessions.get(session_id) {
            Some(session) => {
                if session.is_expired() {
                    Err(AuthError::SessionExpired)
                } else {
                    Ok(session.to_response())
                }
            }
            None => Err(AuthError::SessionNotFound),
        }
    }

    /// Déconnecte un utilisateur en supprimant sa session
    ///
    /// # Arguments
    /// * `session_id` - L'identifiant de session à supprimer
    ///
    /// # Returns
    /// * `Ok(LogoutResponse)` - Indique si la session a été trouvée et supprimée
    pub fn logout(&self, session_id: &str) -> Result<LogoutResponse, AuthError> {
        let mut sessions = self.sessions.lock().map_err(|_| AuthError::StorageError)?;

        let session_found = sessions.remove(session_id).is_some();

        Ok(LogoutResponse { session_found })
    }

    /// Nettoie les sessions expirées (peut être appelé périodiquement)
    #[allow(dead_code)]
    pub fn cleanup_expired_sessions(&self) -> Result<usize, AuthError> {
        let mut sessions = self.sessions.lock().map_err(|_| AuthError::StorageError)?;

        let initial_count = sessions.len();
        sessions.retain(|_, session| !session.is_expired());
        let cleaned = initial_count - sessions.len();

        Ok(cleaned)
    }

    /// Retourne le nombre de sessions actives (non expirées)
    #[allow(dead_code)]
    pub fn active_sessions_count(&self) -> Result<usize, AuthError> {
        let sessions = self.sessions.lock().map_err(|_| AuthError::StorageError)?;

        let count = sessions
            .values()
            .filter(|session| !session.is_expired())
            .count();

        Ok(count)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}
