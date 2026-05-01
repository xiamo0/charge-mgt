//! BootNotification 响应

use crate::common::status::RegistrationStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BootNotificationConfirmation {
    pub status: RegistrationStatus,
    pub current_time: String,
    pub interval: i32,
}

impl BootNotificationConfirmation {
    pub fn accepted(current_time: &str, interval_secs: i32) -> Self {
        Self {
            status: RegistrationStatus::Accepted,
            current_time: current_time.to_string(),
            interval: interval_secs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_notification_confirmation_accepted() {
        let conf = BootNotificationConfirmation::accepted("2024-01-01T00:00:00Z", 30);
        assert_eq!(conf.status, RegistrationStatus::Accepted);
        assert_eq!(conf.current_time, "2024-01-01T00:00:00Z");
        assert_eq!(conf.interval, 30);
    }

    #[test]
    fn test_boot_notification_confirmation_roundtrip() {
        let conf = BootNotificationConfirmation::accepted("2024-12-31T23:59:59Z", 300);
        let json = serde_json::to_string(&conf).unwrap();
        let de: BootNotificationConfirmation = serde_json::from_str(&json).unwrap();
        assert_eq!(conf.status, de.status);
        assert_eq!(conf.current_time, de.current_time);
        assert_eq!(conf.interval, de.interval);
    }

    #[test]
    fn test_boot_notification_confirmation_pending_status() {
        let conf = BootNotificationConfirmation {
            status: RegistrationStatus::Pending,
            current_time: "2024-01-01T00:00:00Z".to_string(),
            interval: 120,
        };
        let de: BootNotificationConfirmation =
            serde_json::from_str(&serde_json::to_string(&conf).unwrap()).unwrap();
        assert_eq!(de.status, RegistrationStatus::Pending);
    }
}
