mod control;
mod model;
pub mod data_provider;

pub fn configure_document_routes() -> actix_web::Scope {
    actix_web::web::scope("/api/documents")
        .route("", actix_web::web::get().to(control::get_documents))
        .route("", actix_web::web::post().to(control::create_document))
        .route("/{id}", actix_web::web::get().to(control::get_document_by_id))
}
