use std::sync::Mutex;
use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, cookie::Cookie};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc, Duration};



struct AppState {
    app_name: String,
    counter: Mutex<i32>,
    sessions: Mutex<HashMap<String, Session>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix-web server on http://127.0.0.1:8080");
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
            .supports_credentials() // Nécessaire pour les cookies
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("api/auth")
                    .route("/login", web::post().to(log_in))
                    .route("/logout", web::post().to(log_out))
                    .route("/verify", web::get().to(verify_session))
            )
            .route("/hey", web::get().to(manuel_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn manuel_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Manuel!")
}


#[derive(Deserialize)]
struct LoginCredentials {
    user: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    success: bool,
}

#[derive(Clone)]
struct Session {
    user_id: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

// Fonction pour encoder le mot de passe en Base64 (comme côté front-end)
fn encode_password(password: &str) -> String {
    general_purpose::STANDARD.encode(password.as_bytes())
}

// Fonction pour valider les credentials
fn validate_credentials(user: &str, password: &str) -> bool {
    // Validation simple : user = "test", password = hash de "test"
    let expected_password_hash = encode_password("test");
    user == "test" && password == expected_password_hash
}

// Login handler
async fn log_in(
    credentials: web::Json<LoginCredentials>,
    data: web::Data<AppState>,
) -> impl Responder {
    // 1- récupérer le body contenant les credentials
    let creds = credentials.into_inner();
    
    // 2- valider les credentials
    if !validate_credentials(&creds.user, &creds.password) {
        return HttpResponse::Unauthorized().json(LoginResponse {
            message: "Invalid credentials".to_string(),
            success: false,
        });
    }
    
    // 3- créer une session
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let session = Session {
        user_id: creds.user.clone(),
        created_at: now,
        expires_at: now + chrono::Duration::hours(24), // Session expire dans 24h
    };
    
    // Stocker la session
    if let Ok(mut sessions) = data.sessions.lock() {
        sessions.insert(session_id.clone(), session);
    }
    
    // 4- retourner une réponse avec un cookie HTTP-only
    let cookie = Cookie::build("session_id", session_id)
        .http_only(true) // Temporairement false pour debug - mettre true en production
        .secure(true) // Mettre à true en production avec HTTPS
        .same_site(actix_web::cookie::SameSite::Lax) // Important pour CORS
        .path("/")
        .max_age(actix_web::cookie::time::Duration::hours(24))
        .finish();
    
    HttpResponse::Ok()
        .cookie(cookie)
        .json(LoginResponse {
            message: "Login successful".to_string(),
            success: true,
        })
}

// Endpoint pour vérifier si une session est valide
async fn verify_session(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // Récupérer le cookie de session
    let session_id = match req.cookie("session_id") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpResponse::Unauthorized().json(LoginResponse {
                message: "No session cookie found".to_string(),
                success: false,
            });
        }
    };
    
    // Vérifier si la session existe et est valide
    if let Ok(sessions) = data.sessions.lock() {
        if let Some(session) = sessions.get(&session_id) {
            if session.expires_at > Utc::now() {
                return HttpResponse::Ok().json(LoginResponse {
                    message: format!("Session valid for user: {}", session.user_id),
                    success: true,
                });
            }
        }
    }
    
    HttpResponse::Unauthorized().json(LoginResponse {
        message: "Invalid or expired session".to_string(),
        success: false,
    })
}

async fn log_out(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // Récupérer le cookie de session
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        
        // Supprimer la session du stockage
        if let Ok(mut sessions) = data.sessions.lock() {
            sessions.remove(&session_id);
        }
        
        // Supprimer le cookie côté client
        let expire_cookie = Cookie::build("session_id", "")
            .http_only(false) // Même config que lors de la création
            .same_site(actix_web::cookie::SameSite::Lax)
            .path("/")
            .max_age(actix_web::cookie::time::Duration::seconds(0))
            .finish();
        
        return HttpResponse::Ok()
            .cookie(expire_cookie)
            .json(LoginResponse {
                message: "Logout successful".to_string(),
                success: true,
            });
    }
    
    HttpResponse::Ok().json(LoginResponse {
        message: "No active session found".to_string(),
        success: true,
    })
}