use super::data_provider::{DataProviderError, DocumentDataProvider};
use super::model::{CreateDocumentRequest, DocumentMongo, DocumentResponse, UpdateDocumentRequest};
use mongodb::Database;

pub struct DocumentService {
    data_provider: DocumentDataProvider,
}

impl DocumentService {
    pub async fn new(database: &Database) -> Result<Self, mongodb::error::Error> {
        // Initialiser la collection documents
        super::db::initialize_documents_collection(database).await?;

        Ok(Self {
            data_provider: DocumentDataProvider::new(database),
        })
    }

    // ============ Méthodes publiques du service ============

    pub async fn create_document(
        &self,
        request: CreateDocumentRequest,
    ) -> Result<DocumentResponse, DataProviderError> {
        let mongo_document: DocumentMongo = request.into();
        let doc_id = self.data_provider.insert_document(mongo_document).await?;

        // Récupérer le document créé
        let created_doc = self
            .data_provider
            .find_document_by_id(&doc_id)
            .await?
            .expect("Document should exist after creation");

        Ok(created_doc.to_response())
    }

    pub async fn get_all_documents(&self) -> Result<Vec<DocumentResponse>, DataProviderError> {
        let mongo_documents = self.data_provider.find_documents().await?;
        let responses = mongo_documents
            .into_iter()
            .map(|doc| doc.to_response())
            .collect();
        Ok(responses)
    }

    pub async fn get_document_by_id(
        &self,
        doc_id: &str,
    ) -> Result<Option<DocumentResponse>, DataProviderError> {
        let mongo_doc = self.data_provider.find_document_by_id(doc_id).await?;
        Ok(mongo_doc.map(|doc| doc.to_response()))
    }

    pub async fn update_document(
        &self,
        doc_id: &str,
        request: UpdateDocumentRequest,
    ) -> Result<Option<DocumentResponse>, DataProviderError> {
        let updated = self
            .data_provider
            .update_document(doc_id, request.content)
            .await?;

        if updated {
            // Récupérer le document mis à jour
            let updated_doc = self.data_provider.find_document_by_id(doc_id).await?;
            Ok(updated_doc.map(|doc| doc.to_response()))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_document(&self, doc_id: &str) -> Result<bool, DataProviderError> {
        self.data_provider.delete_document(doc_id).await
    }
}
