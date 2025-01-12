use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::Error as SmtpError,
};

#[derive(Clone)]
pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl EmailService {
    pub fn new(email: String, password: String) -> Self {
        let creds = Credentials::new(email.clone(), password);
        
        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        EmailService {
            smtp_transport,
            from_email: email,
        }
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), SmtpError> {
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        self.smtp_transport.send(email).await?;
        Ok(())
    }
} 