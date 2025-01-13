use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::Error as SmtpError,
    message::{MultiPart, SinglePart},
    message::header::{ContentType, ContentDisposition},
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::fmt;

#[derive(Clone)]
pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    user_name: String,
}

#[derive(Debug)]
pub enum EmailError {
    SmtpError(SmtpError),
    IoError(std::io::Error),
    NoPdfProvided,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::SmtpError(err) => write!(f, "SMTP error: {}", err),
            EmailError::IoError(err) => write!(f, "IO error: {}", err),
            EmailError::NoPdfProvided => write!(f, "No PDF files provided"),
        }
    }
}

impl std::error::Error for EmailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EmailError::SmtpError(err) => Some(err),
            EmailError::IoError(err) => Some(err),
            EmailError::NoPdfProvided => None,
        }
    }
}

impl From<SmtpError> for EmailError {
    fn from(err: SmtpError) -> Self {
        EmailError::SmtpError(err)
    }
}

impl From<std::io::Error> for EmailError {
    fn from(err: std::io::Error) -> Self {
        EmailError::IoError(err)
    }
}

impl EmailService {
    pub fn new(email: String, name: String, password: String) -> Self {
        let creds = Credentials::new(email.clone(), password);
        
        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        EmailService {
            smtp_transport,
            from_email: email,
            user_name: name,
        }
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        pdf_paths: Vec<PathBuf>,
    ) -> Result<(), EmailError> {
        if pdf_paths.is_empty() {
            return Err(EmailError::NoPdfProvided);
        }

        // Use the first PDF found
        let pdf_path = &pdf_paths[0];
        let mut file = File::open(pdf_path)?;
        let mut pdf_data = Vec::new();
        file.read_to_end(&mut pdf_data)?;

        let filename = pdf_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}-resume.pdf", self.user_name));

        let email = Message::builder()
            .from(format!("{} <{}>", self.user_name, self.from_email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(String::from(body))
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::parse("application/pdf").unwrap())
                            .header(ContentDisposition::attachment(&filename))
                            .body(pdf_data)
                    ),
            )
            .unwrap();

        self.smtp_transport.send(email).await?;
        Ok(())
    }
}
