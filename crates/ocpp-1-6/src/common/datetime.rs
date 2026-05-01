//! OCPP 日期时间类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcppDateTime(pub String);

impl OcppDateTime {
    pub fn now() -> Self {
        Self(chrono::Utc::now().to_rfc3339())
    }

    pub fn from_rfc3339(s: &str) -> Option<Self> {
        if chrono::DateTime::parse_from_rfc3339(s).is_ok() {
            Some(Self(s.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for OcppDateTime {
    fn default() -> Self {
        Self::now()
    }
}
