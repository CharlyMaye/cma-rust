use actix_web::web;

use crate::authentication::control::{log_in, log_out, verify_session};

pub mod control;
pub mod model;
pub mod service;
mod utils;

pub use service::AuthService;

/// Configure et retourne le scope d'authentification avec toutes les routes
pub fn configure_auth_routes() -> actix_web::Scope {
    web::scope("api/auth")
        .route("/login", web::post().to(log_in))
        .route("/logout", web::post().to(log_out))
        .route("/verify", web::get().to(verify_session))
}
