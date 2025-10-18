use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod authentication;
mod documents;
mod model;
mod config;
mod db;

use crate::model::AppState;
use crate::config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    
    println!("Starting Actix-web server on http://{}:{}", config.server.host, config.server.port);
    
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

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(authentication::configure_auth_routes())
            .service(documents::configure_document_routes())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}

