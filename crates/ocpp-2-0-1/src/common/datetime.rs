//! DateTime Utilities for OCPP 2.0.1

/// 获取当前时间的 RFC 3339 字符串
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// 将时间戳转换为 RFC 3339 字符串
pub fn to_rfc3339(timestamp: i64) -> String {
    chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "invalid-timestamp".to_string())
}

/// 解析 RFC 3339 字符串为时间戳
pub fn from_rfc3339(s: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp())
}