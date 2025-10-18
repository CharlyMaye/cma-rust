use actix_web::{web, HttpResponse, Result};
use utoipa;

use crate::model::AppState;
use crate::common::{ApiResponse, ErrorResponse};
use super::model::{CreateDocumentRequest, UpdateDocumentRequest, DocumentResponse};

/// Récupère tous les documents
/// 
/// Retourne la liste complète des documents stockés dans la base de données.
/// La réponse inclut le nombre total de documents dans les métadonnées.
#[utoipa::path(
    get,
    path = "/api/documents",
    tag = "Documents",
    responses(
        (status = 200, description = "Liste des documents récupérée avec succès", body = inline(ApiResponse<Vec<DocumentResponse>>)),
        (status = 500, description = "Erreur serveur interne", body = ErrorResponse)
    )
)]
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

/// Récupère un document par son identifiant
/// 
/// Recherche et retourne un document spécifique en utilisant son doc_id.
#[utoipa::path(
    get,
    path = "/api/documents/{id}",
    tag = "Documents",
    params(
        ("id" = String, Path, description = "Identifiant unique du document (doc_id)")
    ),
    responses(
        (status = 200, description = "Document trouvé", body = inline(ApiResponse<DocumentResponse>)),
        (status = 404, description = "Document non trouvé", body = ErrorResponse),
        (status = 500, description = "Erreur serveur interne", body = ErrorResponse)
    )
)]
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

/// Crée un nouveau document
/// 
/// Insère un nouveau document dans la base de données. Le doc_id doit être unique.
/// En cas de succès, retourne le document créé avec un header Location.
#[utoipa::path(
    post,
    path = "/api/documents",
    tag = "Documents",
    request_body = CreateDocumentRequest,
    responses(
        (status = 201, description = "Document créé avec succès", body = inline(ApiResponse<DocumentResponse>),
            headers(
                ("Location" = String, description = "URI du document créé")
            )
        ),
        (status = 409, description = "Un document avec ce doc_id existe déjà", body = ErrorResponse),
        (status = 500, description = "Erreur serveur interne", body = ErrorResponse)
    )
)]
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

/// Met à jour un document existant
/// 
/// Modifie le contenu d'un document identifié par son doc_id.
/// Le doc_id lui-même ne peut pas être modifié.
#[utoipa::path(
    put,
    path = "/api/documents/{id}",
    tag = "Documents",
    params(
        ("id" = String, Path, description = "Identifiant unique du document (doc_id)")
    ),
    request_body = UpdateDocumentRequest,
    responses(
        (status = 200, description = "Document mis à jour avec succès", body = inline(ApiResponse<DocumentResponse>)),
        (status = 404, description = "Document non trouvé", body = ErrorResponse),
        (status = 500, description = "Erreur serveur interne", body = ErrorResponse)
    )
)]
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

/// Supprime un document
/// 
/// Supprime définitivement un document identifié par son doc_id.
/// Retourne 204 No Content en cas de succès (pas de corps de réponse).
#[utoipa::path(
    delete,
    path = "/api/documents/{id}",
    tag = "Documents",
    params(
        ("id" = String, Path, description = "Identifiant unique du document (doc_id)")
    ),
    responses(
        (status = 204, description = "Document supprimé avec succès"),
        (status = 404, description = "Document non trouvé", body = ErrorResponse),
        (status = 500, description = "Erreur serveur interne", body = ErrorResponse)
    )
)]
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