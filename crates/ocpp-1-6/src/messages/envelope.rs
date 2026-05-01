//! 消息信封类型

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OCPP CALL 消息: [2, "<uniqueId>", "<action>", {payload}]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Call {
    #[serde(rename = "2")]
    pub message_type_id: i32,
    #[serde(rename = "3")]
    pub unique_id: String,
    #[serde(rename = "4")]
    pub action: String,
    #[serde(rename = "5")]
    pub payload: Value,
}

impl Call {
    pub fn new(action: &str, unique_id: &str, payload: Value) -> Self {
        Self {
            message_type_id: 2,
            unique_id: unique_id.to_string(),
            action: action.to_string(),
            payload,
        }
    }
}

/// OCPP CALLRESULT 消息: [3, "<uniqueId>", {payload}]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallResult {
    #[serde(rename = "3")]
    pub message_type_id: i32,
    #[serde(rename = "4")]
    pub unique_id: String,
    #[serde(rename = "5")]
    pub payload: Value,
}

impl CallResult {
    pub fn new(unique_id: &str, payload: Value) -> Self {
        Self {
            message_type_id: 3,
            unique_id: unique_id.to_string(),
            payload,
        }
    }
}

/// OCPP CALLERROR 消息: [4, "<uniqueId>", "<errorCode>", "<errorDescription>", {errorDetails}]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallError {
    #[serde(rename = "4")]
    pub message_type_id: i32,
    #[serde(rename = "5")]
    pub unique_id: String,
    #[serde(rename = "6")]
    pub error_code: String,
    #[serde(rename = "7")]
    pub error_description: String,
    #[serde(rename = "8")]
    pub error_details: Value,
}

impl CallError {
    pub fn new(unique_id: &str, error_code: &str, error_description: &str) -> Self {
        Self {
            message_type_id: 4,
            unique_id: unique_id.to_string(),
            error_code: error_code.to_string(),
            error_description: error_description.to_string(),
            error_details: Value::Null,
        }
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.error_details = details;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Call 测试
    #[test]
    fn test_call_new() {
        let payload = serde_json::json!({"idTag": "TAG001"});
        let call = Call::new("Authorize", "uuid-123", payload.clone());
        assert_eq!(call.message_type_id, 2);
        assert_eq!(call.unique_id, "uuid-123");
        assert_eq!(call.action, "Authorize");
        assert_eq!(call.payload, payload);
    }

    #[test]
    fn test_call_deserialization() {
        let json = r#"[2,"uuid-456","BootNotification",{"chargePointVendor":"VendorX","chargePointModel":"ModelY"}]"#;
        let de: Call = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 2);
        assert_eq!(de.unique_id, "uuid-456");
        assert_eq!(de.action, "BootNotification");
    }

    #[test]
    fn test_call_roundtrip() {
        let payload = serde_json::json!({"key": "value"});
        let call = Call::new("Heartbeat", "uuid-789", payload);
        let json = serde_json::to_string(&call).unwrap();
        let de: Call = serde_json::from_str(&json).unwrap();
        assert_eq!(call.message_type_id, de.message_type_id);
        assert_eq!(call.unique_id, de.unique_id);
        assert_eq!(call.action, de.action);
    }

    // CallResult 测试
    #[test]
    fn test_call_result_new() {
        let payload = serde_json::json!({"status": "Accepted"});
        let result = CallResult::new("uuid-123", payload.clone());
        assert_eq!(result.message_type_id, 3);
        assert_eq!(result.unique_id, "uuid-123");
        assert_eq!(result.payload, payload);
    }

    #[test]
    fn test_call_result_deserialization() {
        let json = r#"[3,"uuid-789",{"status":"Accepted"}]"#;
        let de: CallResult = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 3);
        assert_eq!(de.unique_id, "uuid-789");
    }

    #[test]
    fn test_call_result_roundtrip() {
        let payload = serde_json::json!({"transactionId": 42});
        let result = CallResult::new("uuid-123", payload);
        let json = serde_json::to_string(&result).unwrap();
        let de: CallResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result.message_type_id, de.message_type_id);
        assert_eq!(result.unique_id, de.unique_id);
    }

    // CallError 测试
    #[test]
    fn test_call_error_new() {
        let error = CallError::new("uuid-123", "NotImplemented", "Action not supported");
        assert_eq!(error.message_type_id, 4);
        assert_eq!(error.unique_id, "uuid-123");
        assert_eq!(error.error_code, "NotImplemented");
        assert_eq!(error.error_description, "Action not supported");
        assert_eq!(error.error_details, Value::Null);
    }

    #[test]
    fn test_call_error_with_details() {
        let details = serde_json::json!({"info": "custom error info"});
        let error = CallError::new("uuid-123", "ProtocolError", "Something went wrong")
            .with_details(details.clone());
        assert_eq!(error.error_details, details);
    }

    #[test]
    fn test_call_error_deserialization() {
        let json = r#"[4,"uuid-456","InternalError","Internal server error",{}]"#;
        let de: CallError = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 4);
        assert_eq!(de.unique_id, "uuid-456");
        assert_eq!(de.error_code, "InternalError");
        assert_eq!(de.error_description, "Internal server error");
    }

    #[test]
    fn test_call_error_roundtrip() {
        let details = serde_json::json!({"error_details": {"code": 123}});
        let error = CallError::new("uuid-789", "FormationViolation", "Invalid message format")
            .with_details(details.clone());
        let json = serde_json::to_string(&error).unwrap();
        let de: CallError = serde_json::from_str(&json).unwrap();
        assert_eq!(error.message_type_id, de.message_type_id);
        assert_eq!(error.unique_id, de.unique_id);
        assert_eq!(error.error_code, de.error_code);
        assert_eq!(error.error_details, details);
    }

    #[test]
    fn test_call_error_message_type_id_is_4() {
        let error = CallError::new("uuid-123", "NotImplemented", "desc");
        assert_eq!(error.message_type_id, 4);
    }
}
