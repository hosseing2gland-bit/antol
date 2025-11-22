use lettre::{
    Message, SmtpTransport, Transport,
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use std::env;

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse::<u16>()?;
        let smtp_user = env::var("SMTP_USER")?;
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let from_email = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@antidetect.local".to_string());

        let creds = Credentials::new(smtp_user, smtp_password);

        let mailer = SmtpTransport::relay(&smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(EmailService { mailer, from_email })
    }

    pub fn send_welcome_email(&self, to: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("Welcome to Anti-Detect Browser")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #4F46E5;">Welcome to Anti-Detect Browser!</h2>
                        <p>Hello <strong>{}</strong>,</p>
                        <p>Your account has been successfully created. You can now access all features of our anti-detection browser platform.</p>
                        <h3>Getting Started:</h3>
                        <ul>
                            <li>Create browser profiles with unique fingerprints</li>
                            <li>Configure proxy settings for each profile</li>
                            <li>Launch browsers with anti-detection features</li>
                            <li>Manage multiple accounts safely</li>
                        </ul>
                        <p style="margin-top: 30px;">
                            <a href="http://localhost:8080" style="background: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block;">
                                Access Admin Panel
                            </a>
                        </p>
                        <p style="margin-top: 30px; color: #666; font-size: 12px;">
                            If you didn't create this account, please contact support immediately.
                        </p>
                    </div>
                </body>
                </html>
                "#,
                username
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_license_activated(&self, to: &str, license_key: &str, expires_at: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("License Activated - Anti-Detect Browser")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #10B981;">License Activated Successfully!</h2>
                        <p>Your license has been activated and is now ready to use.</p>
                        <div style="background: #F3F4F6; padding: 15px; border-radius: 5px; margin: 20px 0;">
                            <p style="margin: 5px 0;"><strong>License Key:</strong> <code>{}</code></p>
                            <p style="margin: 5px 0;"><strong>Expires:</strong> {}</p>
                        </div>
                        <h3>What's Next?</h3>
                        <ul>
                            <li>Download and install the client application</li>
                            <li>Enter your license key in the activation screen</li>
                            <li>Start creating browser profiles</li>
                        </ul>
                        <p style="margin-top: 30px; color: #EF4444; font-weight: bold;">
                            ⚠️ Keep your license key secure and do not share it with others.
                        </p>
                    </div>
                </body>
                </html>
                "#,
                license_key, expires_at
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_license_expiring_soon(&self, to: &str, license_key: &str, days_remaining: i32) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("License Expiring Soon - Anti-Detect Browser")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #F59E0B;">License Expiring Soon!</h2>
                        <p>Your license will expire in <strong>{} days</strong>.</p>
                        <div style="background: #FEF3C7; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #F59E0B;">
                            <p style="margin: 5px 0;"><strong>License Key:</strong> <code>{}</code></p>
                            <p style="margin: 5px 0;"><strong>Days Remaining:</strong> {}</p>
                        </div>
                        <p>To avoid service interruption, please renew your license before it expires.</p>
                        <p style="margin-top: 30px;">
                            <a href="http://localhost:8080/licenses" style="background: #F59E0B; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block;">
                                Renew License
                            </a>
                        </p>
                    </div>
                </body>
                </html>
                "#,
                days_remaining, license_key, days_remaining
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_license_expired(&self, to: &str, license_key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("License Expired - Anti-Detect Browser")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #EF4444;">License Expired</h2>
                        <p>Your license has expired and can no longer be used.</p>
                        <div style="background: #FEE2E2; padding: 15px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #EF4444;">
                            <p style="margin: 5px 0;"><strong>License Key:</strong> <code>{}</code></p>
                            <p style="margin: 5px 0;"><strong>Status:</strong> Expired</p>
                        </div>
                        <p>To continue using our services, please purchase a new license.</p>
                        <p style="margin-top: 30px;">
                            <a href="http://localhost:8080/licenses" style="background: #EF4444; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block;">
                                Get New License
                            </a>
                        </p>
                    </div>
                </body>
                </html>
                "#,
                license_key
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_password_reset(&self, to: &str, reset_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let reset_link = format!("http://localhost:8080/reset-password?token={}", reset_token);
        
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject("Password Reset Request - Anti-Detect Browser")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #4F46E5;">Password Reset Request</h2>
                        <p>We received a request to reset your password.</p>
                        <p>Click the button below to reset your password. This link will expire in 1 hour.</p>
                        <p style="margin-top: 30px;">
                            <a href="{}" style="background: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block;">
                                Reset Password
                            </a>
                        </p>
                        <p style="margin-top: 30px; color: #666; font-size: 12px;">
                            If you didn't request this, please ignore this email. Your password will remain unchanged.
                        </p>
                        <p style="color: #666; font-size: 12px;">
                            Or copy this link: <code>{}</code>
                        </p>
                    </div>
                </body>
                </html>
                "#,
                reset_link, reset_link
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Run manually with proper SMTP credentials
    fn test_send_welcome_email() {
        let service = EmailService::new().unwrap();
        service.send_welcome_email("test@example.com", "TestUser").unwrap();
    }
}
