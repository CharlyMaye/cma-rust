pub mod control;
mod data_provider;
mod db;
pub mod model;
mod service;

// Exporter uniquement le service et les types publics nécessaires
pub use data_provider::DataProviderError;
pub use service::DocumentService;

/// Configure les routes de l'API Documents
///
/// Note: Pour appliquer le middleware d'authentification, utilisez .wrap(AuthMiddleware)
/// au niveau de l'App ou du scope parent dans main.rs
pub fn configure_document_routes() -> actix_web::Scope {
    actix_web::web::scope("/api/documents")
        .route("", actix_web::web::get().to(control::get_documents))
        .route("", actix_web::web::post().to(control::create_document))
        .route(
            "/{id}",
            actix_web::web::get().to(control::get_document_by_id),
        )
        .route("/{id}", actix_web::web::put().to(control::update_document))
        .route(
            "/{id}",
            actix_web::web::delete().to(control::delete_document),
        )
}
