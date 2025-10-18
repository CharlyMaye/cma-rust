use actix_web::{web, HttpResponse, Responder, HttpRequest, cookie::Cookie};
use uuid::Uuid;
use chrono::Utc;
use serde::Serialize;
use utoipa::{self, ToSchema};

use crate::{
    authentication::{model::{LoginCredentials, SessionData}, utils, Session}, 
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
    request_body = LoginCredentials,
    responses(
        (status = 200, description = "Authentification réussie - Cookie session_id défini", 
            body = inline(ApiResponse<SessionData>),
            headers(
                ("Set-Cookie" = String, description = "Cookie de session (session_id)")
            )
        ),
        (status = 401, description = "Identifiants invalides", body = ErrorResponse)
    )
)]
pub async fn log_in(
    credentials: web::Json<LoginCredentials>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("🚀 POST /api/auth/login - Tentative de connexion");
    
    // 1- récupérer le body contenant les credentials
    let creds = credentials.into_inner();
    println!("   👤 User: {}", creds.user);
    
    // 2- valider les credentials
    if !utils::validate_credentials(&creds.user, &creds.password) {
        println!("   ❌ Échec d'authentification pour {}", creds.user);
        let error = ErrorResponse::new("Invalid credentials");
        return HttpResponse::Unauthorized().json(error);
    }
    
    println!("   ✅ Authentification réussie pour {}", creds.user);
    
    // 3- créer une session
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let session = Session {
        user_id: creds.user.clone(),
        created_at: now,
        expires_at: now + chrono::Duration::hours(24), // Session expire dans 24h
    };
    
    let session_data = session.to_data();
    
    // Stocker la session
    if let Ok(mut sessions) = data.sessions.lock() {
        sessions.insert(session_id.clone(), session);
        println!("   💾 Session créée: {}", session_id);
    }
    
    // 4- retourner une réponse avec un cookie HTTP-only
    let cookie = Cookie::build("session_id", session_id.clone())
        .http_only(false) // Temporairement false pour debug - mettre true en production
        .secure(false) // IMPORTANT: false pour HTTP, true seulement pour HTTPS
        .same_site(actix_web::cookie::SameSite::Lax) // Lax pour cross-site mais même domain
        .domain("localhost") // Spécifier explicitement le domain
        .path("/")
        .max_age(actix_web::cookie::time::Duration::hours(24))
        .finish();
    
    println!("   🍪 Cookie créé: session_id={} (Max-Age: 24h, SameSite: Lax, Secure: false)", session_id);
    
    let response = ApiResponse::success_with_message(session_data, "Login successful");
    
    HttpResponse::Ok()
        .cookie(cookie)
        .json(response)
}

/// Vérifie si une session est valide
/// 
/// Valide le cookie de session et retourne les informations de session si valide.
#[utoipa::path(
    get,
    path = "/api/auth/verify",
    tag = "Authentication",
    responses(
        (status = 200, description = "Session valide", body = inline(ApiResponse<SessionData>)),
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

    
    // Vérifier si la session existe et est valide
    if let Ok(sessions) = data.sessions.lock() {
        if let Some(session) = sessions.get(&session_id) {
            if session.expires_at > Utc::now() {
                println!("   ✅ Session valide pour: {}", session.user_id);
                let session_data = session.to_data();
                let response = ApiResponse::success(session_data);
                return HttpResponse::Ok().json(response);
            } else {
                println!("   ⏰ Session expirée pour: {}", session.user_id);
            }
        } else {
            println!("   ❌ Session introuvable: {}", session_id);
        }
    }
    
    let error = ErrorResponse::new("Invalid or expired session");
    HttpResponse::Unauthorized().json(error)
}

/// Déconnexion de l'utilisateur
/// 
/// Invalide la session côté serveur et supprime le cookie.
#[derive(Serialize, ToSchema)]
struct LogoutData {
    /// Indique si une session a été trouvée
    #[schema(example = true)]
    session_found: bool,
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Déconnexion réussie", body = inline(ApiResponse<LogoutData>)),
    ),
    security(
        ("session_cookie" = [])
    )
)]
pub async fn log_out(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("🚪 POST /api/auth/logout - Tentative de déconnexion");
    
    // Récupérer le cookie de session
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        println!("   🍪 Session à supprimer: {}", session_id);
        
        // Supprimer la session du stockage
        if let Ok(mut sessions) = data.sessions.lock() {
            if sessions.remove(&session_id).is_some() {
                println!("   ✅ Session supprimée avec succès");
            } else {
                println!("   ⚠️  Session introuvable dans le stockage");
            }
        }
        
        // Supprimer le cookie côté client
        let expire_cookie = Cookie::build("session_id", "")
            .http_only(false) // Même config que lors de la création
            .secure(false) // Même config que lors de la création
            .same_site(actix_web::cookie::SameSite::Lax)
            .domain("localhost")
            .path("/")
            .max_age(actix_web::cookie::time::Duration::seconds(0))
            .finish();
        
        let response = ApiResponse::success_with_message(
            LogoutData { session_found: true },
            "Logout successful"
        );
        
        return HttpResponse::Ok()
            .cookie(expire_cookie)
            .json(response);
    }
    
    println!("   ⚠️  Aucun cookie de session trouvé pour logout");
    let response = ApiResponse::success_with_message(
        LogoutData { session_found: false },
        "No active session found"
    );
    HttpResponse::Ok().json(response)
}
