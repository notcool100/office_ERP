use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use anyhow::Result;
use std::env;

pub struct Mailer {
    transport: SmtpTransport,
    from: String,
}

impl Mailer {
    pub fn new() -> Self {
        let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set").parse().expect("SMTP_PORT must be a number");
        let user = env::var("SMTP_USER").expect("SMTP_USER must be set");
        let pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");

        let creds = Credentials::new(user, pass);

        // For local development and internal VPS communication, we might have self-signed certificates.
        // We use Tls::Required but can disable certificate verification if needed, 
        // or just use Tls::None if on localhost.
        let transport = if host == "127.0.0.1" || host == "localhost" {
            SmtpTransport::builder_dangerous(host)
                .port(port)
                .credentials(creds)
                .tls(lettre::transport::smtp::client::Tls::None)
                .build()
        } else {
            SmtpTransport::starttls_relay(&host)
                .unwrap()
                .port(port)
                .credentials(creds)
                // Relax certificate verification for this specific setup where mail.ubucknepal.com 
                // certificate might have issues when accessed via STARTTLS from the same machine.
                .tls(lettre::transport::smtp::client::Tls::Required(
                    lettre::transport::smtp::client::TlsParameters::builder(host)
                        .dangerous_accept_invalid_certs(true)
                        .dangerous_accept_invalid_hostnames(true)
                        .build()
                        .unwrap()
                ))
                .build()
        };

        Self { transport, from }
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(body.to_string())?;

        match self.transport.send(&email) {
            Ok(_) => {
                println!("[MAILER] Email successfully sent to {}", to);
                Ok(())
            }
            Err(e) => {
                eprintln!("[MAILER] Error sending email to {}: {:?}", to, e);
                Err(e.into())
            }
        }
    }

    pub fn send_welcome_email(&self, to: &str, username: &str, temp_pass: &str) -> Result<()> {
        let subject = "Welcome to ubuck ERP!";
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #333; text-align: center;">Welcome to the Team!</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>Your account for the <strong>ubuck ERP</strong> has been successfully created.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 0;"><strong>Username:</strong> {}</p>
                    <p style="margin: 0;"><strong>Temporary Password:</strong> <span style="color: #d9534f; font-family: monospace;">{}</span></p>
                </div>
                <p>Please log in and change your password immediately for security.</p>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/login" style="background-color: #0275d8; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">Login to ERP</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            username, username, temp_pass
        );

        self.send_email(to, subject, &body)
    }

    pub fn send_task_assignment_email(
        &self,
        to: &str,
        assignee_name: &str,
        task_title: &str,
        project_name: &str,
        priority: &str,
    ) -> Result<()> {
        let subject = format!("New Task Assigned: {}", task_title);
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #333; text-align: center;">New Task Assigned</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>You have been assigned a new task in project <strong>{}</strong>.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 0;"><strong>Task:</strong> {}</p>
                    <p style="margin: 0;"><strong>Priority:</strong> <span style="text-transform: capitalize;">{}</span></p>
                </div>
                <p>Please review the task details and update the status accordingly.</p>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/admin/projects" style="background-color: #0275d8; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">View Task</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            assignee_name, project_name, task_title, priority
        );

        self.send_email(to, &subject, &body)
    }

    pub fn send_broadcast_email(
        &self,
        to_list: Vec<String>,
        subject: &str,
        title: &str,
        content: &str,
    ) -> Result<()> {
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #d9534f; text-align: center;">{}</h2>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd; line-height: 1.6;">
                    {}
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an official announcement from ubuck ERP.</p>
            </div>
            "#,
            title, content
        );

        println!("[MAILER] Starting broadcast to {} recipients", to_list.len());
        for (i, to) in to_list.iter().enumerate() {
            if let Err(e) = self.send_email(to, subject, &body) {
                eprintln!("[MAILER] Broadcast failed for recipient {} ({}): {:?}", i+1, to, e);
            }
        }
        println!("[MAILER] Broadcast complete");
        Ok(())
    }

}
