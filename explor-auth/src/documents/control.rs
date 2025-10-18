use actix_web::{web, HttpResponse, Result};
use serde_json::json;

use crate::model::AppState;

pub async fn get_documents(data: web::Data<AppState>) -> Result<HttpResponse> {
    let data_provider = data.get_document_provider();
    
    match data_provider.find_documents().await {
        Ok(documents) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": documents,
            "count": documents.len()
        }))),
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
        Ok(Some(document)) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": document
        }))),
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
    body: web::Json<serde_json::Value>,
    data: web::Data<AppState>
) -> Result<HttpResponse> {
    let data_provider = data.get_document_provider();
    
    // Convertir le JSON en Document MongoDB
    let document = match mongodb::bson::to_document(&body.into_inner()) {
        Ok(doc) => doc,
        Err(err) => {
            eprintln!("Error converting JSON to Document: {:?}", err);
            return Ok(HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Invalid document format"
            })));
        }
    };
    
    match data_provider.insert_document(document).await {
        Ok(inserted_id) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "message": "Document created successfully",
            "id": inserted_id
        }))),
        Err(err) => {
            eprintln!("Error creating document: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to create document"
            })))
        }
    }
}