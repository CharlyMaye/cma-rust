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

// DTO Response - Ce qui est retourné par l'API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentResponse {
    pub id: String,          // _id MongoDB en hex
    pub doc_id: String,      // Identifiant métier unique
    pub content: String,
}

// DTO Request - Pour créer un document
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    pub doc_id: String,
    pub content: String,
}

// DTO Request - Pour mettre à jour un document
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDocumentRequest {
    pub content: String,     // On ne peut pas changer le doc_id (c'est l'identifiant unique)
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
            id: self.id
                .as_ref()
                .map(|oid| oid.to_hex())
                .unwrap_or_else(|| "".to_string()),
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
