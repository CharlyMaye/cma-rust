use serde::Serialize;
use utoipa::ToSchema;

/// Structure de réponse API standardisée
///
/// Toutes les réponses de succès suivent ce format avec des données
/// et des métadonnées séparées.
#[derive(Debug, Serialize, ToSchema)]
#[serde(bound = "T: Serialize")]
pub struct ApiResponse<T: Serialize> {
    /// Les données de la réponse
    pub data: T,

    /// Métadonnées sur la réponse (status, count, etc.)
    pub metadata: ResponseMetadata,
}

/// Métadonnées de la réponse
///
/// Contient des informations sur le statut de la requête et optionnellement
/// un message ou un compteur d'éléments.
#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseMetadata {
    /// Statut de la réponse ("success" ou "error")
    #[schema(example = "success")]
    pub status: String,

    /// Message optionnel (principalement pour les erreurs)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "Operation completed successfully")]
    pub message: Option<String>,

    /// Nombre d'éléments retournés (pour les listes)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = 42)]
    pub count: Option<usize>,
}

impl ResponseMetadata {
    pub fn success() -> Self {
        Self {
            status: "success".to_string(),
            message: None,
            count: None,
        }
    }

    pub fn success_with_message(message: impl Into<String>) -> Self {
        Self {
            status: "success".to_string(),
            message: Some(message.into()),
            count: None,
        }
    }

    pub fn success_with_count(count: usize) -> Self {
        Self {
            status: "success".to_string(),
            message: None,
            count: Some(count),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            message: Some(message.into()),
            count: None,
        }
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            metadata: ResponseMetadata::success(),
        }
    }

    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            metadata: ResponseMetadata::success_with_message(message),
        }
    }

    pub fn success_with_count(data: T, count: usize) -> Self {
        Self {
            data,
            metadata: ResponseMetadata::success_with_count(count),
        }
    }
}

/// Structure pour les réponses d'erreur (quand il n'y a pas de data)
///
/// Utilisée pour retourner uniquement un message d'erreur sans données.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Métadonnées contenant le statut "error" et le message
    pub metadata: ResponseMetadata,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            metadata: ResponseMetadata::error(message),
        }
    }
}
