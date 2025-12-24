use crate::error::AppError;

use tracing::info;

pub struct NotificationService;

impl NotificationService {
    pub async fn send_email(to_email: &str, subject: &str, body: &str) -> Result<(), AppError> {
        info!(
            "📧 [MOCK EMAIL] To: {}, Subject: {}, Body: {}",
            to_email, subject, body
        );

        Ok(())
    }
}
