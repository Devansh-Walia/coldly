use actix_web::{get, App, HttpResponse, HttpServer, Responder, post, Error, error, web};
use serde::Serialize;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

mod config;
mod models;
use crate::config::Settings;
mod services;
use crate::services::{process_csv, EmailService};

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

#[post("/upload-csv")]
async fn upload_csv(
    mut payload: Multipart,
    email_service: web::Data<EmailService>,
) -> Result<HttpResponse, Error> {
    // Process the multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Skip if not a file
        if let Some(content_type) = field.content_type() {
            if content_type.essence_str() == "multipart/form-data" {
                continue;
            }
        }

        // Create a temporary file
        let mut temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| error::ErrorInternalServerError(e))?;

        // Read file bytes and write to temp file
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| error::ErrorInternalServerError(e))?;
            temp_file
                .write_all(&data)
                .map_err(|e| error::ErrorInternalServerError(e))?;
        }


        // Process the CSV file
        let records = process_csv(temp_file.reopen().map_err(|e| error::ErrorInternalServerError(e))?)
            .map_err(|e| error::ErrorInternalServerError(e))?;

        // Send email notification
        email_service
            .send_email(
                "some-email@gmail.com",
                "CSV Processing Complete",
                &format!("Processed {} records from CSV file", records.len()),
            )
            .await
            .map_err(|e| error::ErrorInternalServerError(e))?;

        return Ok(HttpResponse::Ok().json(records));
    }

    Err(error::ErrorBadRequest("No file provided"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let settings = Settings::new().expect("Failed to load settings");
    let email_service = web::Data::new(EmailService::new(
        settings.smtp_email,
        settings.smtp_password,
    ));
    
    println!("Server starting at http://127.0.0.1:{}", settings.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(email_service.clone())
            .service(health_check)
            .service(upload_csv)
    })
    .bind(("127.0.0.1", settings.port))?
    .run()
    .await
}
