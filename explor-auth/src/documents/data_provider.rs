use futures::TryStreamExt;
use mongodb::{Collection, Database};

use super::model::DocumentMongo;

#[derive(Debug, Clone)]
pub struct DocumentDataProvider {
    collection: Collection<DocumentMongo>,
}

#[derive(Debug)]
pub enum DataProviderError {
    MongoError(mongodb::error::Error),
    InvalidObjectId(String),
    SerializationError(mongodb::bson::ser::Error),
    DuplicateDocId(String),
}

impl From<mongodb::error::Error> for DataProviderError {
    fn from(error: mongodb::error::Error) -> Self {
        DataProviderError::MongoError(error)
    }
}

impl From<mongodb::bson::ser::Error> for DataProviderError {
    fn from(error: mongodb::bson::ser::Error) -> Self {
        DataProviderError::SerializationError(error)
    }
}

impl DocumentDataProvider {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection::<DocumentMongo>("documents");
        Self { collection }
    }

    pub async fn insert_document(&self, doc: DocumentMongo) -> Result<String, DataProviderError> {
        use mongodb::bson::doc;

        // Vérifier si doc_id existe déjà
        let existing = self
            .collection
            .find_one(doc! { "doc_id": &doc.doc_id })
            .await?;

        if existing.is_some() {
            return Err(DataProviderError::DuplicateDocId(doc.doc_id.clone()));
        }

        let result = self.collection.insert_one(doc).await?;
        Ok(result.inserted_id.to_string())
    }

    pub async fn find_documents(&self) -> Result<Vec<DocumentMongo>, DataProviderError> {
        use mongodb::bson::doc;

        let cursor = self.collection.find(doc! {}).await?;
        let documents: Vec<DocumentMongo> = cursor.try_collect().await?;
        Ok(documents)
    }

    pub async fn find_document_by_id(
        &self,
        doc_id: &str,
    ) -> Result<Option<DocumentMongo>, DataProviderError> {
        use mongodb::bson::doc;

        let filter = doc! { "doc_id": doc_id };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn update_document(
        &self,
        doc_id: &str,
        content: String,
    ) -> Result<bool, DataProviderError> {
        use mongodb::bson::doc;

        let filter = doc! { "doc_id": doc_id };
        let update_doc = doc! {
            "$set": {
                "content": content,
            }
        };

        let result = self.collection.update_one(filter, update_doc).await?;
        Ok(result.modified_count > 0)
    }

    pub async fn delete_document(&self, doc_id: &str) -> Result<bool, DataProviderError> {
        use mongodb::bson::doc;

        let filter = doc! { "doc_id": doc_id };
        let result = self.collection.delete_one(filter).await?;
        Ok(result.deleted_count > 0)
    }
}
