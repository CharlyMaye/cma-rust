use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
    counter: Mutex<i32>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix-web server on http://127.0.0.1:8080");
    let app_state = web::Data::new(AppState {
        app_name: "My Actix-web App".into(),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("api/auth")
                    .route("/login", web::post().to(log_in))
                    .route("/logout", web::post().to(log_out))
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

// Login handler
async fn log_in() -> impl Responder {
    HttpResponse::Ok().body("{\"message\": \"Login endpoint\"}")
}
async fn log_out() -> impl Responder {
    HttpResponse::Ok().body("{\"message\": \"Logout endpoint\"}")
}