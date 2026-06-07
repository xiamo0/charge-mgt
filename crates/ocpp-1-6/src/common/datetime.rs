//! OCPP 日期时间类型
//!
//! 提供 OCPP 中常用的时间类型封装，内部使用 ISO 8601 / RFC3339 字符串

use serde::{Deserialize, Serialize};

/// OCPP 日期时间的简单封装（内部为 RFC3339 字符串）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcppDateTime(pub String);

impl OcppDateTime {
    /// 返回当前时间的 `OcppDateTime` 实例（UTC，RFC3339 格式）
    pub fn now() -> Self {
        Self(chrono::Utc::now().to_rfc3339())
    }

    /// 从 RFC3339 字符串解析为 `OcppDateTime`，解析失败返回 None
    pub fn from_rfc3339(s: &str) -> Option<Self> {
        if chrono::DateTime::parse_from_rfc3339(s).is_ok() {
            Some(Self(s.to_string()))
        } else {
            None
        }
    }

    /// 以 &str 形式返回内部时间字符串
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for OcppDateTime {
    /// 默认值为当前时间（now）
    fn default() -> Self {
        Self::now()
    }
}
