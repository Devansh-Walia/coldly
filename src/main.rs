use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
mod config;
use crate::config::Settings;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "ok".to_string(),
        message: "Service is healthy".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let settings = Settings::new().expect("Failed to load settings");
    
    println!("Server starting at http://127.0.0.1:{}", settings.port);
    
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind(("127.0.0.1", settings.port))?
    .run()
    .await
} 