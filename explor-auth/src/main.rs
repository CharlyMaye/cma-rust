use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod authentication;
mod documents;
mod model;
mod config;
mod db;
mod middleware;

use crate::model::AppState;
use crate::config::Config;
use crate::documents::model::{DocumentResponse, CreateDocumentRequest, UpdateDocumentRequest};
use crate::documents::response::{ApiResponse, ErrorResponse, ResponseMetadata};

/// Spécification OpenAPI pour l'API Documents
#[derive(OpenApi)]
#[openapi(
    paths(
        documents::control::get_documents,
        documents::control::get_document_by_id,
        documents::control::create_document,
        documents::control::update_document,
        documents::control::delete_document,
    ),
    components(
        schemas(
            DocumentResponse,
            CreateDocumentRequest,
            UpdateDocumentRequest,
            ApiResponse<DocumentResponse>,
            ApiResponse<Vec<DocumentResponse>>,
            ErrorResponse,
            ResponseMetadata,
        )
    ),
    tags(
        (name = "Documents", description = "API de gestion des documents")
    ),
    info(
        title = "Documents API",
        version = "1.0.0",
        description = "API REST pour la gestion des documents avec authentification",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    
    println!("🚀 Starting Actix-web server on http://{}:{}", config.server.host, config.server.port);
    println!("📖 Swagger UI available at http://{}:{}/swagger-ui/", config.server.host, config.server.port);
    
    // Initialiser l'AppState avec MongoDB
    let app_state = web::Data::new(
        AppState::new()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
    );

    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    let cors_origin = config.cors.allowed_origin.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            // Nécessaire pour les cookies
            .supports_credentials()
            .max_age(3600);

        // Créer la spécification OpenAPI
        let openapi = ApiDoc::openapi();
        
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            // Swagger UI - accessible à http://localhost:8080/swagger-ui/
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            // Routes d'authentification (pas de middleware)
            .service(authentication::configure_auth_routes())
            // Routes documents avec middleware d'authentification
            .service(
                documents::configure_document_routes()
                    .wrap(middleware::AuthMiddleware)
            )
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}

