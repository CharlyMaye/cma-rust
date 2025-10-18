use serde::Serialize;

/// Structure de réponse API standardisée
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub metadata: ResponseMetadata,
}

/// Métadonnées de la réponse
#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            metadata: ResponseMetadata::success(),
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
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub metadata: ResponseMetadata,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            metadata: ResponseMetadata::error(message),
        }
    }
}
