use std::sync::Mutex;
use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

pub mod authentication;
pub mod model;
use authentication::configure_auth_routes;

use crate::model::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix-web server on http://localhost:8080");
    let app_state = web::Data::new(AppState {
        app_name: "My Actix-web App".into(),
        counter: Mutex::new(0),
        sessions: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST"])
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
            .service(configure_auth_routes())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

