use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod authentication;
mod common;
mod config;
mod db;
mod documents;
mod middleware;
mod model;

use crate::authentication::model::{LoginRequest, LogoutResponse, SessionResponse};
use crate::common::{ApiResponse, ErrorResponse, ResponseMetadata};
use crate::config::Config;
use crate::documents::model::{CreateDocumentRequest, DocumentResponse, UpdateDocumentRequest};
use crate::model::AppState;

/// Spécification OpenAPI pour l'API complète
#[derive(OpenApi)]
#[openapi(
    paths(
        // Routes d'authentification
        authentication::control::log_in,
        authentication::control::verify_session,
        authentication::control::log_out,
        // Routes documents
        documents::control::get_documents,
        documents::control::get_document_by_id,
        documents::control::create_document,
        documents::control::update_document,
        documents::control::delete_document,
    ),
    components(
        schemas(
            // Modèles d'authentification
            LoginRequest,
            SessionResponse,
            LogoutResponse,
            // Modèles de documents
            DocumentResponse,
            CreateDocumentRequest,
            UpdateDocumentRequest,
            // Réponses API
            ApiResponse<SessionResponse>,
            ApiResponse<LogoutResponse>,
            ApiResponse<DocumentResponse>,
            ApiResponse<Vec<DocumentResponse>>,
            ErrorResponse,
            ResponseMetadata,
        )
    ),
    tags(
        (name = "Authentication", description = "Authentification et gestion des sessions"),
        (name = "Documents", description = "API de gestion des documents (requiert authentification)")
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Explor Auth API",
        version = "1.0.0",
        description = "API REST avec authentification par cookie de session",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

/// Configuration de sécurité pour Swagger UI
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::*;

        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session_id"))),
            );
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    println!(
        "🚀 Starting Actix-web server on http://{}:{}",
        config.server.host, config.server.port
    );
    println!(
        "📖 Swagger UI available at http://{}:{}/swagger-ui/",
        config.server.host, config.server.port
    );

    // Initialiser l'AppState avec MongoDB
    let app_state = web::Data::new(
        AppState::new()
            .await
            .map_err(std::io::Error::other)?,
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
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // Routes d'authentification (pas de middleware)
            .service(authentication::configure_auth_routes())
            // Routes documents avec middleware d'authentification
            .service(documents::configure_document_routes().wrap(middleware::AuthMiddleware))
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}
