use std::sync::Mutex;

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
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .service(
                web::scope("/app")
                .route("/index.html", web::get().to(index))
            )
            .route("/hey", web::get().to(manuel_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello from Actix-web!")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
async fn manuel_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Manuel!")
}


async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    let response = format!("Welcome to {}'s index page! Request number: {}", app_name, counter);
    HttpResponse::Ok().body(response)
}