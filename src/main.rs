use actix_web::{get, App, HttpResponse, HttpServer, Responder, post, Error, error, web};
use serde::Serialize;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

mod config;
mod models;
use crate::config::Settings;
mod services;
use crate::services::{process_csv, EmailService, read_file_to_string, get_attachment};

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

#[post("/send-emails")]
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

        let mut count_sent = 0;
        let mut count_failed = 0;

        let subject = read_file_to_string("src/templates/email_subject.txt")
            .map_err(|e| error::ErrorInternalServerError(e))?;
        let body_template = read_file_to_string("src/templates/email_template.txt")
            .map_err(|e| error::ErrorInternalServerError(e))?;
        let pdf_file_paths = get_attachment("src/templates")?;

        let is_empty = pdf_file_paths.is_empty();

        if is_empty {
            return Err(error::ErrorInternalServerError("No resume files found"));
        }

        for record in records {
            let body = body_template.replace("{lead_name}", &record.first_name);

            match email_service
                .send_email(&record.email, &subject, &body, pdf_file_paths.clone())
                .await
            {
                Ok(_) => count_sent += 1,
                Err(e) => {
                    eprintln!("Failed to send email to {}: {:?}", record.email, e);
                    count_failed += 1;
                }
            }
        }

        return Ok(HttpResponse::Ok().json(format!(
            "Emails sent: {}, Failed: {}",count_sent, count_failed
        )));
    }

    Err(error::ErrorBadRequest("No file provided"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let settings = Settings::new().expect("Failed to load settings");
    let email_service = web::Data::new(EmailService::new(
        settings.smtp_email,
        settings.smtp_user_name,
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
