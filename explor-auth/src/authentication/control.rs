use actix_web::{web, HttpResponse, Responder, HttpRequest, cookie::Cookie};
use utoipa;

use crate::{
    authentication::{
        model::{LoginRequest, SessionResponse, LogoutResponse},
        service::AuthError,
    }, 
    model::AppState,
    common::{ApiResponse, ErrorResponse},
};



/// Authentification de l'utilisateur
/// 
/// Valide les identifiants et crée une session. Retourne un cookie `session_id`
/// qui doit être utilisé pour les requêtes authentifiées.
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authentification réussie - Cookie session_id défini", 
            body = inline(ApiResponse<SessionResponse>),
            headers(
                ("Set-Cookie" = String, description = "Cookie de session (session_id)")
            )
        ),
        (status = 401, description = "Identifiants invalides", body = ErrorResponse)
    )
)]
pub async fn log_in(
    request: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("🚀 POST /api/auth/login - Tentative de connexion");
    
    let login_request = request.into_inner();
    let username = login_request.user.clone();
    println!("   👤 User: {}", username);
    
    // Appeler le service d'authentification
    match data.auth_service.login(login_request) {
        Ok((session_id, session_response)) => {
            println!("   ✅ Authentification réussie pour {}", username);
            println!("   💾 Session créée: {}", session_id);
            
            // Créer le cookie de session
            let cookie = Cookie::build("session_id", session_id.clone())
                .http_only(false) // Temporairement false pour debug - mettre true en production
                .secure(false) // IMPORTANT: false pour HTTP, true seulement pour HTTPS
                .same_site(actix_web::cookie::SameSite::Lax)
                .domain("localhost")
                .path("/")
                .max_age(actix_web::cookie::time::Duration::hours(24))
                .finish();
            
            println!("   🍪 Cookie créé: session_id={} (Max-Age: 24h, SameSite: Lax, Secure: false)", session_id);
            
            let response = ApiResponse::success_with_message(session_response, "Login successful");
            
            HttpResponse::Ok()
                .cookie(cookie)
                .json(response)
        }
        Err(AuthError::InvalidCredentials) => {
            println!("   ❌ Échec d'authentification pour {}", username);
            let error = ErrorResponse::new("Invalid credentials");
            HttpResponse::Unauthorized().json(error)
        }
        Err(_) => {
            println!("   ⚠️  Erreur serveur lors de l'authentification");
            let error = ErrorResponse::new("Internal server error");
            HttpResponse::InternalServerError().json(error)
        }
    }
}

/// Vérifie si une session est valide
/// 
/// Valide le cookie de session et retourne les informations de session si valide.
#[utoipa::path(
    get,
    path = "/api/auth/verify",
    tag = "Authentication",
    responses(
        (status = 200, description = "Session valide", body = inline(ApiResponse<SessionResponse>)),
        (status = 401, description = "Session invalide ou expirée", body = ErrorResponse)
    ),
    security(
        ("session_cookie" = [])
    )
)]
pub async fn verify_session(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("🔍 GET /api/auth/verify - Vérification de session");
    
    // Récupérer le cookie de session
    let session_id = match req.cookie("session_id") {
        Some(cookie) => {
            let id = cookie.value().to_string();
            println!("   🍪 Cookie trouvé: {}", id);
            id
        },
        None => {
            println!("   ❌ Aucun cookie de session trouvé");
            let error = ErrorResponse::new("No session cookie found");
            return HttpResponse::Unauthorized().json(error);
        }
    };

    
    // Vérifier la session via le service
    match data.auth_service.verify_session(&session_id) {
        Ok(session_response) => {
            println!("   ✅ Session valide pour: {}", session_response.user_id);
            let response = ApiResponse::success(session_response);
            HttpResponse::Ok().json(response)
        }
        Err(AuthError::SessionNotFound) => {
            println!("   ❌ Session introuvable: {}", session_id);
            let error = ErrorResponse::new("Invalid or expired session");
            HttpResponse::Unauthorized().json(error)
        }
        Err(AuthError::SessionExpired) => {
            println!("   ⏰ Session expirée");
            let error = ErrorResponse::new("Invalid or expired session");
            HttpResponse::Unauthorized().json(error)
        }
        Err(_) => {
            println!("   ⚠️  Erreur serveur lors de la vérification");
            let error = ErrorResponse::new("Internal server error");
            HttpResponse::InternalServerError().json(error)
        }
    }
}

/// Déconnexion de l'utilisateur
/// 
/// Supprime la session et invalide le cookie.
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Déconnexion réussie", body = inline(ApiResponse<LogoutResponse>)),
    )
)]
pub async fn log_out(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("🚪 POST /api/auth/logout - Déconnexion");
    
    // Récupérer le cookie session_id
    let session_id = match req.cookie("session_id") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            println!("   ℹ️  Aucun cookie session_id trouvé - déjà déconnecté");
            let logout_response = LogoutResponse { session_found: false };
            let response = ApiResponse::success_with_message(
                logout_response,
                "No active session found"
            );
            return HttpResponse::Ok().json(response);
        }
    };
    
    println!("   🔑 Session ID: {}", session_id);
    
    // Supprimer la session via le service
    match data.auth_service.logout(&session_id) {
        Ok(logout_response) => {
            if logout_response.session_found {
                println!("   ✅ Session supprimée");
            } else {
                println!("   ⚠️  Session ID non trouvé dans le store");
            }
            
            // Invalider le cookie
            let cookie = Cookie::build("session_id", "")
                .path("/")
                .max_age(actix_web::cookie::time::Duration::seconds(0))
                .finish();
            
            println!("   🍪 Cookie invalidé");
            
            let message = if logout_response.session_found {
                "Logout successful"
            } else {
                "No active session found"
            };
            
            let response = ApiResponse::success_with_message(logout_response, message);
            HttpResponse::Ok()
                .cookie(cookie)
                .json(response)
        }
        Err(_) => {
            println!("   ⚠️  Erreur serveur lors de la déconnexion");
            let error = ErrorResponse::new("Internal server error");
            HttpResponse::InternalServerError().json(error)
        }
    }
}
