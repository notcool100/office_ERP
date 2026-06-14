use anyhow::Result;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub struct Mailer {
    transport: SmtpTransport,
    from: String,
}

impl Default for Mailer {
    fn default() -> Self {
        Self::new()
    }
}

impl Mailer {
    pub fn new() -> Self {
        let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let port = env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse()
            .expect("SMTP_PORT must be a number");
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
                        .unwrap(),
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

    pub fn send_leave_submitted_email(
        &self,
        to_list: Vec<String>,
        employee_name: &str,
        leave_type: &str,
        start_date: &str,
        end_date: &str,
        reason: &str,
    ) -> Result<()> {
        let subject = format!("Leave Request from {}", employee_name);
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #333; text-align: center;">New Leave Request</h2>
                <p>A new leave request has been submitted and requires your approval.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 5px 0;"><strong>Employee:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Leave Type:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>From:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>To:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Reason:</strong> {}</p>
                </div>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/admin/leave" style="background-color: #0275d8; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">Review Request</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            employee_name, leave_type, start_date, end_date, reason
        );
        for to in &to_list {
            if let Err(e) = self.send_email(to, &subject, &body) {
                eprintln!("[MAILER] Leave submitted notification failed for {}: {:?}", to, e);
            }
        }
        Ok(())
    }

    pub fn send_leave_decision_email(
        &self,
        to: &str,
        employee_name: &str,
        leave_type: &str,
        start_date: &str,
        end_date: &str,
        approved: bool,
        notes: Option<&str>,
    ) -> Result<()> {
        let (status_label, color) = if approved {
            ("Approved", "#5cb85c")
        } else {
            ("Rejected", "#d9534f")
        };
        let subject = format!("Leave Request {}", status_label);
        let notes_html = notes
            .filter(|n| !n.is_empty())
            .map(|n| format!("<p style=\"margin: 5px 0;\"><strong>Notes:</strong> {}</p>", n))
            .unwrap_or_default();
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: {}; text-align: center;">Leave Request {}</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>Your leave request has been <strong style="color: {};">{}</strong>.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 5px 0;"><strong>Leave Type:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>From:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>To:</strong> {}</p>
                    {}
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            color, status_label, employee_name, color, status_label.to_lowercase(),
            leave_type, start_date, end_date, notes_html
        );
        self.send_email(to, &subject, &body)
    }

    pub fn send_project_member_added_email(
        &self,
        to: &str,
        member_name: &str,
        project_name: &str,
        role: &str,
    ) -> Result<()> {
        let subject = format!("You've been added to project: {}", project_name);
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #333; text-align: center;">Added to Project</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>You have been added to the project <strong>{}</strong> as a <strong style="text-transform: capitalize;">{}</strong>.</p>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/admin/projects" style="background-color: #0275d8; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">View Project</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            member_name, project_name, role
        );
        self.send_email(to, &subject, &body)
    }

    pub fn send_due_date_reminder_email(
        &self,
        to: &str,
        assignee_name: &str,
        task_title: &str,
        project_name: &str,
        due_date: &str,
    ) -> Result<()> {
        let subject = format!("Task Due Tomorrow: {}", task_title);
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #f0ad4e; text-align: center;">Task Due Reminder</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>The following task is due <strong>tomorrow ({})</strong>. Please make sure it's on track.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 5px 0;"><strong>Task:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Project:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Due Date:</strong> {}</p>
                </div>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/admin/projects" style="background-color: #f0ad4e; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">View Task</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            assignee_name, due_date, task_title, project_name, due_date
        );
        self.send_email(to, &subject, &body)
    }

    pub fn send_schedule_reminder_email(
        &self,
        to: &str,
        user_name: &str,
        note_title: &str,
        event_date: &str,
    ) -> Result<()> {
        let subject = format!("Schedule Reminder: {}", note_title);
        let body = format!(
            r#"
            <div style="font-family: Arial, sans-serif; max-width: 600px; margin: auto; padding: 20px; border: 1px solid #eee; border-radius: 10px; background-color: #f9f9f9;">
                <h2 style="color: #5bc0de; text-align: center;">Schedule Reminder</h2>
                <p>Hello <strong>{}</strong>,</p>
                <p>This is a reminder for your scheduled note <strong>tomorrow ({})</strong>.</p>
                <div style="background-color: #fff; padding: 15px; border-radius: 5px; margin: 20px 0; border: 1px solid #ddd;">
                    <p style="margin: 5px 0;"><strong>Note:</strong> {}</p>
                    <p style="margin: 5px 0;"><strong>Date:</strong> {}</p>
                </div>
                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://ubucknepal.com/admin/profile/schedule" style="background-color: #5bc0de; color: white; padding: 12px 25px; text-decoration: none; border-radius: 5px; font-weight: bold;">View Schedule</a>
                </div>
                <hr style="border: 0; border-top: 1px solid #ddd; margin: 30px 0;">
                <p style="font-size: 12px; color: #777; text-align: center;">This is an automated message from ubuck ERP. Please do not reply.</p>
            </div>
            "#,
            user_name, event_date, note_title, event_date
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

        println!(
            "[MAILER] Starting broadcast to {} recipients",
            to_list.len()
        );
        for (i, to) in to_list.iter().enumerate() {
            if let Err(e) = self.send_email(to, subject, &body) {
                eprintln!(
                    "[MAILER] Broadcast failed for recipient {} ({}): {:?}",
                    i + 1,
                    to,
                    e
                );
            }
        }
        println!("[MAILER] Broadcast complete");
        Ok(())
    }
}
