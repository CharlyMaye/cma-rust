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

// DTO pour l'API (externe)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub doc_id: String,
    pub content: String,
}

impl DocumentMongo {
    pub fn new(doc_id: String, content: String) -> Self {
        Self {
            id: None, // MongoDB générera automatiquement l'_id
            doc_id,
            content,
        }
    }

    // Convertir de DocumentMongo vers Document (DTO)
    pub fn to_document(&self) -> Document {
        Document {
            id: self.id
                .as_ref()
                .map(|oid| oid.to_hex())
                .unwrap_or_else(|| "".to_string()),
            doc_id: self.doc_id.clone(),
            content: self.content.clone(),
        }
    }
}

impl Document {
    // Convertir de Document (DTO) vers DocumentMongo
    pub fn to_mongo(&self) -> DocumentMongo {
        DocumentMongo {
            id: None, // L'ID sera géré par MongoDB
            doc_id: self.doc_id.clone(),
            content: self.content.clone(),
        }
    }
}

// Request pour créer un document
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    pub doc_id: String,
    pub content: String,
}

impl From<CreateDocumentRequest> for DocumentMongo {
    fn from(req: CreateDocumentRequest) -> Self {
        DocumentMongo::new(req.doc_id, req.content)
    }
}
