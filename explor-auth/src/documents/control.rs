use actix_web::{web, HttpResponse, Result};
use serde_json::json;

use crate::model::AppState;
use super::model::{CreateDocumentRequest, UpdateDocumentRequest, DocumentResponse};

pub async fn get_documents(data: web::Data<AppState>) -> Result<HttpResponse> {
    let data_provider = data.get_document_provider();
    
    match data_provider.find_documents().await {
        Ok(mongo_documents) => {
            // Convertir DocumentMongo -> DocumentResponse (DTO)
            let documents: Vec<DocumentResponse> = mongo_documents
                .into_iter()
                .map(|doc| doc.to_response())
                .collect();
            
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "data": documents,
                "count": documents.len()
            })))
        },
        Err(err) => {
            eprintln!("Error fetching documents: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch documents"
            })))
        }
    }
}

pub async fn get_document_by_id(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let document_id = path.into_inner();
    let data_provider = data.get_document_provider();
    
    match data_provider.find_document_by_id(&document_id).await {
        Ok(Some(mongo_doc)) => {
            // Convertir DocumentMongo -> DocumentResponse (DTO)
            let document = mongo_doc.to_response();
            
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "data": document
            })))
        },
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Document not found"
        }))),
        Err(err) => {
            eprintln!("Error fetching document: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch document"
            })))
        }
    }
}

pub async fn create_document(
    body: web::Json<CreateDocumentRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let data_provider = data.get_document_provider();
    
    // Convertir CreateDocumentRequest -> DocumentMongo
    let mongo_document = body.into_inner().into();
    
    match data_provider.insert_document(mongo_document).await {
        Ok(inserted_id) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "message": "Document created successfully",
            "id": inserted_id
        }))),
        Err(super::data_provider::DataProviderError::DuplicateDocId(doc_id)) => {
            Ok(HttpResponse::Conflict().json(json!({
                "status": "error",
                "message": format!("Document with doc_id '{}' already exists", doc_id)
            })))
        },
        Err(err) => {
            eprintln!("Error creating document: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create document"
            })))
        }
    }
}

pub async fn update_document(
    path: web::Path<String>,
    body: web::Json<UpdateDocumentRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let doc_id = path.into_inner();
    let data_provider = data.get_document_provider();
    let request = body.into_inner();
    
    match data_provider.update_document(&doc_id, request.content).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Document updated successfully"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Document not found"
        }))),
        Err(err) => {
            eprintln!("Error updating document: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to update document"
            })))
        }
    }
}

pub async fn delete_document(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let doc_id = path.into_inner();
    let data_provider = data.get_document_provider();
    
    match data_provider.delete_document(&doc_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Document deleted successfully"
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Document not found"
        }))),
        Err(err) => {
            eprintln!("Error deleting document: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to delete document"
            })))
        }
    }
}