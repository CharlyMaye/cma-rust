use mongodb::{bson::Document, Collection, Database};
use futures::TryStreamExt;

#[derive(Debug, Clone)]
pub struct DocumentDataProvider {
    collection: Collection<Document>,
}

#[derive(Debug)]
pub enum DataProviderError {
    MongoError(mongodb::error::Error),
    InvalidObjectId(String),
}

impl From<mongodb::error::Error> for DataProviderError {
    fn from(error: mongodb::error::Error) -> Self {
        DataProviderError::MongoError(error)
    }
}

impl DocumentDataProvider {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection("documents");
        Self { collection }
    }

    pub async fn insert_document(&self, doc: Document) -> Result<String, DataProviderError> {
        let result = self.collection.insert_one(doc).await?;
        Ok(result.inserted_id.to_string())
    }

    pub async fn find_documents(&self) -> Result<Vec<Document>, DataProviderError> {
        let cursor = self.collection.find(Document::new()).await?;
        let documents: Vec<Document> = cursor.try_collect().await?;
        Ok(documents)
    }

    pub async fn find_document_by_id(&self, id: &str) -> Result<Option<Document>, DataProviderError> {
        use mongodb::bson::{doc, oid::ObjectId};
        
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| DataProviderError::InvalidObjectId(id.to_string()))?;
            
        let filter = doc! { "_id": object_id };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }

    pub async fn update_document(&self, id: &str, update: Document) -> Result<bool, DataProviderError> {
        use mongodb::bson::{doc, oid::ObjectId};
        
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| DataProviderError::InvalidObjectId(id.to_string()))?;
            
        let filter = doc! { "_id": object_id };
        let update_doc = doc! { "$set": update };
        
        let result = self.collection.update_one(filter, update_doc).await?;
        Ok(result.modified_count > 0)
    }

    pub async fn delete_document(&self, id: &str) -> Result<bool, DataProviderError> {
        use mongodb::bson::{doc, oid::ObjectId};
        
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| DataProviderError::InvalidObjectId(id.to_string()))?;
            
        let filter = doc! { "_id": object_id };
        let result = self.collection.delete_one(filter).await?;
        Ok(result.deleted_count > 0)
    }
}