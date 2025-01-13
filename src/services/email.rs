use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::Error as SmtpError,
};

#[derive(Clone)]
pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    user_name: String
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
            user_name: name
        }
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), SmtpError> {
        let email = Message::builder()
            .from(format!("{} <{}>",self.user_name, self.from_email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        self.smtp_transport.send(email).await?;
        Ok(())
    }
} 