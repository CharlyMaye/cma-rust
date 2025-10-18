use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Modèle MongoDB (interne)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Généré automatiquement par MongoDB à l'insertion

    pub doc_id: String,
    pub content: String,
}

// ============ DTOs (Data Transfer Objects) ============

use utoipa::ToSchema;

/// Représentation d'un document retourné par l'API
///
/// Contient l'identifiant MongoDB interne et l'identifiant métier unique.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DocumentResponse {
    /// Identifiant MongoDB (ObjectId en hexadécimal)
    #[schema(example = "507f1f77bcf86cd799439011")]
    pub id: String,

    /// Identifiant métier unique du document
    #[schema(example = "DOC-2025-001")]
    pub doc_id: String,

    /// Contenu du document au format JSON stringifié
    #[schema(example = r#"{"title": "Mon document", "description": "Contenu exemple"}"#)]
    pub content: String,
}

/// Requête pour créer un nouveau document
///
/// Le doc_id doit être unique dans la collection.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateDocumentRequest {
    /// Identifiant métier unique du document
    #[schema(example = "DOC-2025-001")]
    pub doc_id: String,

    /// Contenu du document au format JSON stringifié
    #[schema(example = r#"{"title": "Mon document"}"#)]
    pub content: String,
}

/// Requête pour mettre à jour un document existant
///
/// Seul le contenu peut être modifié, le doc_id est immuable.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateDocumentRequest {
    /// Nouveau contenu du document
    #[schema(example = r#"{"title": "Document mis à jour"}"#)]
    pub content: String,
}

// ============ Implementations ============

impl DocumentMongo {
    pub fn new(doc_id: String, content: String) -> Self {
        Self {
            id: None, // MongoDB générera automatiquement l'_id
            doc_id,
            content,
        }
    }

    // Convertir de DocumentMongo vers DocumentResponse (DTO)
    pub fn to_response(&self) -> DocumentResponse {
        DocumentResponse {
            id: self.id.as_ref().map(|oid| oid.to_hex()).unwrap_or_default(),
            doc_id: self.doc_id.clone(),
            content: self.content.clone(),
        }
    }
}

impl From<CreateDocumentRequest> for DocumentMongo {
    fn from(req: CreateDocumentRequest) -> Self {
        DocumentMongo::new(req.doc_id, req.content)
    }
}
