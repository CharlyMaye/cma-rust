use actix_web::{web, HttpResponse, Result};

use crate::model::AppState;
use super::model::{CreateDocumentRequest, UpdateDocumentRequest};
use super::response::{ApiResponse, ErrorResponse};

pub async fn get_documents(data: web::Data<AppState>) -> Result<HttpResponse> {
    match data.document_service.get_all_documents().await {
        Ok(documents) => {
            let count = documents.len();
            let response = ApiResponse::success_with_count(documents, count);
            Ok(HttpResponse::Ok().json(response))
        },
        Err(err) => {
            eprintln!("Error fetching documents: {:?}", err);
            let error = ErrorResponse::new("Failed to fetch documents");
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

pub async fn get_document_by_id(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let document_id = path.into_inner();
    
    match data.document_service.get_document_by_id(&document_id).await {
        Ok(Some(document)) => {
            let response = ApiResponse::success(document);
            Ok(HttpResponse::Ok().json(response))
        },
        Ok(None) => {
            let error = ErrorResponse::new("Document not found");
            Ok(HttpResponse::NotFound().json(error))
        },
        Err(err) => {
            eprintln!("Error fetching document: {:?}", err);
            let error = ErrorResponse::new("Failed to fetch document");
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

pub async fn create_document(
    body: web::Json<CreateDocumentRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let request = body.into_inner();
    
    match data.document_service.create_document(request).await {
        Ok(document) => {
            let location = format!("/api/documents/{}", document.doc_id);
            let response = ApiResponse::success(document);
            Ok(HttpResponse::Created()
                .insert_header(("Location", location))
                .json(response))
        },
        Err(crate::documents::DataProviderError::DuplicateDocId(doc_id)) => {
            let error = ErrorResponse::new(format!("Document with doc_id '{}' already exists", doc_id));
            Ok(HttpResponse::Conflict().json(error))
        },
        Err(err) => {
            eprintln!("Error creating document: {:?}", err);
            let error = ErrorResponse::new("Failed to create document");
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

pub async fn update_document(
    path: web::Path<String>,
    body: web::Json<UpdateDocumentRequest>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let doc_id = path.into_inner();
    let request = body.into_inner();
    
    match data.document_service.update_document(&doc_id, request).await {
        Ok(Some(document)) => {
            let response = ApiResponse::success(document);
            Ok(HttpResponse::Ok().json(response))
        },
        Ok(None) => {
            let error = ErrorResponse::new("Document not found");
            Ok(HttpResponse::NotFound().json(error))
        },
        Err(err) => {
            eprintln!("Error updating document: {:?}", err);
            let error = ErrorResponse::new("Failed to update document");
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}

pub async fn delete_document(
    path: web::Path<String>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let doc_id = path.into_inner();
    
    match data.document_service.delete_document(&doc_id).await {
        Ok(true) => {
            // REST standard: 204 No Content pour une suppression réussie
            Ok(HttpResponse::NoContent().finish())
        },
        Ok(false) => {
            let error = ErrorResponse::new("Document not found");
            Ok(HttpResponse::NotFound().json(error))
        },
        Err(err) => {
            eprintln!("Error deleting document: {:?}", err);
            let error = ErrorResponse::new("Failed to delete document");
            Ok(HttpResponse::InternalServerError().json(error))
        }
    }
}