//! 消息信封类型

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;

/// OCPP CALL 消息: [2, "<uniqueId>", "<action>", {payload}]
#[derive(Debug, Clone)]
pub struct Call {
    pub message_type_id: i32,
    pub unique_id: String,
    pub action: String,
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

impl Serialize for Call {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = serde_json::json!([
            self.message_type_id,
            self.unique_id,
            self.action,
            self.payload
        ]);
        arr.serialize(serializer)
    }
}

struct CallVisitor;

impl<'de> Visitor<'de> for CallVisitor {
    type Value = Call;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an OCPP Call message [2, uniqueId, action, payload]")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v: Value = serde_json::from_str(v).map_err(E::custom)?;
        if let Some(arr) = v.as_array() {
            if arr.len() >= 4 {
                let message_type_id = arr[0]
                    .as_i64()
                    .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
                let unique_id = arr[1]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
                let action = arr[2]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid action"))?;
                let payload = arr[3].clone();
                return Ok(Call {
                    message_type_id: message_type_id as i32,
                    unique_id: unique_id.to_string(),
                    action: action.to_string(),
                    payload,
                });
            }
        }
        if let Some(obj) = v.as_object() {
            let message_type_id = obj
                .get("2")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
            let unique_id = obj
                .get("3")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
            let action = obj
                .get("4")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid action"))?;
            let payload = obj.get("5").cloned().unwrap_or(Value::Null);
            return Ok(Call {
                message_type_id: message_type_id as i32,
                unique_id: unique_id.to_string(),
                action: action.to_string(),
                payload,
            });
        }
        Err(de::Error::custom("invalid OCPP Call message format"))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let message_type_id: i32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing message_type_id"))?;
        let unique_id: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing unique_id"))?;
        let action: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing action"))?;
        let payload: Value = seq.next_element()?.unwrap_or(Value::Null);
        Ok(Call {
            message_type_id,
            unique_id,
            action,
            payload,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut message_type_id = None;
        let mut unique_id = None;
        let mut action = None;
        let mut payload = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "2" => message_type_id = Some(map.next_value()?),
                "3" => unique_id = Some(map.next_value()?),
                "4" => action = Some(map.next_value()?),
                "5" => payload = Some(map.next_value()?),
                _ => {}
            }
        }

        Ok(Call {
            message_type_id: message_type_id
                .ok_or_else(|| de::Error::custom("missing message_type_id"))?,
            unique_id: unique_id.ok_or_else(|| de::Error::custom("missing unique_id"))?,
            action: action.ok_or_else(|| de::Error::custom("missing action"))?,
            payload: payload.unwrap_or(Value::Null),
        })
    }
}

impl<'de> Deserialize<'de> for Call {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CallVisitor)
    }
}

/// OCPP CALLRESULT 消息: [3, "<uniqueId>", {payload}]
#[derive(Debug, Clone)]
pub struct CallResult {
    pub message_type_id: i32,
    pub unique_id: String,
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

impl Serialize for CallResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = serde_json::json!([self.message_type_id, self.unique_id, self.payload]);
        arr.serialize(serializer)
    }
}

struct CallResultVisitor;

impl<'de> Visitor<'de> for CallResultVisitor {
    type Value = CallResult;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an OCPP CallResult message [3, uniqueId, payload]")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v: Value = serde_json::from_str(v).map_err(E::custom)?;
        if let Some(arr) = v.as_array() {
            if arr.len() >= 3 {
                let message_type_id = arr[0]
                    .as_i64()
                    .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
                let unique_id = arr[1]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
                let payload = arr[2].clone();
                return Ok(CallResult {
                    message_type_id: message_type_id as i32,
                    unique_id: unique_id.to_string(),
                    payload,
                });
            }
        }
        if let Some(obj) = v.as_object() {
            let message_type_id = obj
                .get("3")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
            let unique_id = obj
                .get("4")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
            let payload = obj.get("5").cloned().unwrap_or(Value::Null);
            return Ok(CallResult {
                message_type_id: message_type_id as i32,
                unique_id: unique_id.to_string(),
                payload,
            });
        }
        Err(de::Error::custom("invalid OCPP CallResult message format"))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let message_type_id: i32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing message_type_id"))?;
        let unique_id: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing unique_id"))?;
        let payload: Value = seq.next_element()?.unwrap_or(Value::Null);
        Ok(CallResult {
            message_type_id,
            unique_id,
            payload,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut message_type_id = None;
        let mut unique_id = None;
        let mut payload = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "3" => message_type_id = Some(map.next_value()?),
                "4" => unique_id = Some(map.next_value()?),
                "5" => payload = Some(map.next_value()?),
                _ => {}
            }
        }

        Ok(CallResult {
            message_type_id: message_type_id
                .ok_or_else(|| de::Error::custom("missing message_type_id"))?,
            unique_id: unique_id.ok_or_else(|| de::Error::custom("missing unique_id"))?,
            payload: payload.unwrap_or(Value::Null),
        })
    }
}

impl<'de> Deserialize<'de> for CallResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CallResultVisitor)
    }
}

/// OCPP CALLERROR 消息: [4, "<uniqueId>", "<errorCode>", "<errorDescription>", {errorDetails}]
#[derive(Debug, Clone)]
pub struct CallError {
    pub message_type_id: i32,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
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

impl Serialize for CallError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = serde_json::json!([
            self.message_type_id,
            self.unique_id,
            self.error_code,
            self.error_description,
            self.error_details
        ]);
        arr.serialize(serializer)
    }
}

struct CallErrorVisitor;

impl<'de> Visitor<'de> for CallErrorVisitor {
    type Value = CallError;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "an OCPP CallError message [4, uniqueId, errorCode, errorDescription, errorDetails]",
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v: Value = serde_json::from_str(v).map_err(E::custom)?;
        if let Some(arr) = v.as_array() {
            if arr.len() >= 5 {
                let message_type_id = arr[0]
                    .as_i64()
                    .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
                let unique_id = arr[1]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
                let error_code = arr[2]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid error_code"))?;
                let error_description = arr[3]
                    .as_str()
                    .ok_or_else(|| de::Error::custom("invalid error_description"))?;
                let error_details = arr.get(4).cloned().unwrap_or(Value::Null);
                return Ok(CallError {
                    message_type_id: message_type_id as i32,
                    unique_id: unique_id.to_string(),
                    error_code: error_code.to_string(),
                    error_description: error_description.to_string(),
                    error_details,
                });
            }
        }
        if let Some(obj) = v.as_object() {
            let message_type_id = obj
                .get("4")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| de::Error::custom("invalid message_type_id"))?;
            let unique_id = obj
                .get("5")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid unique_id"))?;
            let error_code = obj
                .get("6")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid error_code"))?;
            let error_description = obj
                .get("7")
                .and_then(|v| v.as_str())
                .ok_or_else(|| de::Error::custom("invalid error_description"))?;
            let error_details = obj.get("8").cloned().unwrap_or(Value::Null);
            return Ok(CallError {
                message_type_id: message_type_id as i32,
                unique_id: unique_id.to_string(),
                error_code: error_code.to_string(),
                error_description: error_description.to_string(),
                error_details,
            });
        }
        Err(de::Error::custom("invalid OCPP CallError message format"))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let message_type_id: i32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing message_type_id"))?;
        let unique_id: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing unique_id"))?;
        let error_code: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing error_code"))?;
        let error_description: String = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing error_description"))?;
        let error_details: Value = seq.next_element()?.unwrap_or(Value::Null);
        Ok(CallError {
            message_type_id,
            unique_id,
            error_code,
            error_description,
            error_details,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut message_type_id = None;
        let mut unique_id = None;
        let mut error_code = None;
        let mut error_description = None;
        let mut error_details = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "4" => message_type_id = Some(map.next_value()?),
                "5" => unique_id = Some(map.next_value()?),
                "6" => error_code = Some(map.next_value()?),
                "7" => error_description = Some(map.next_value()?),
                "8" => error_details = Some(map.next_value()?),
                _ => {}
            }
        }

        Ok(CallError {
            message_type_id: message_type_id
                .ok_or_else(|| de::Error::custom("missing message_type_id"))?,
            unique_id: unique_id.ok_or_else(|| de::Error::custom("missing unique_id"))?,
            error_code: error_code.ok_or_else(|| de::Error::custom("missing error_code"))?,
            error_description: error_description
                .ok_or_else(|| de::Error::custom("missing error_description"))?,
            error_details: error_details.unwrap_or(Value::Null),
        })
    }
}

impl<'de> Deserialize<'de> for CallError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CallErrorVisitor)
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
    fn test_call_serialization_array_format() {
        let payload = serde_json::json!({"chargePointVendor":"VendorX"});
        let call = Call::new("BootNotification", "uuid-456", payload);
        let json = serde_json::to_string(&call).unwrap();
        assert!(json.starts_with("["));
        assert!(!json.starts_with("{"));
        assert_eq!(
            json,
            r#"[2,"uuid-456","BootNotification",{"chargePointVendor":"VendorX"}]"#
        );
    }

    #[test]
    fn test_call_deserialization_array_format() {
        let json = r#"[2,"uuid-456","BootNotification",{"chargePointVendor":"VendorX","chargePointModel":"ModelY"}]"#;
        let de: Call = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 2);
        assert_eq!(de.unique_id, "uuid-456");
        assert_eq!(de.action, "BootNotification");
        assert_eq!(de.payload["chargePointVendor"], "VendorX");
    }

    #[test]
    fn test_call_deserialization_object_format() {
        let json = r#"{"2":2,"3":"uuid-456","4":"BootNotification","5":{"chargePointVendor":"VendorX","chargePointModel":"ModelY"}}"#;
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
    fn test_call_result_serialization_array_format() {
        let payload = serde_json::json!({"status":"Accepted","currentTime":"2026-05-02T00:00:00.000Z","interval":30});
        let result = CallResult::new("uuid-123", payload);
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.starts_with("["));
        assert!(!json.starts_with("{"));
        assert!(json.starts_with("[3,\"uuid-123\""));
        assert!(json.contains("\"status\":\"Accepted\""));
        assert!(json.contains("\"interval\":30"));
    }

    #[test]
    fn test_call_result_deserialization_array_format() {
        let json = r#"[3,"uuid-789",{"status":"Accepted"}]"#;
        let de: CallResult = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 3);
        assert_eq!(de.unique_id, "uuid-789");
        assert_eq!(de.payload["status"], "Accepted");
    }

    #[test]
    fn test_call_result_deserialization_object_format() {
        let json = r#"{"3":3,"4":"uuid-789","5":{"status":"Accepted","currentTime":"2026-05-02T00:00:00.000Z"}}"#;
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
    fn test_call_error_serialization_array_format() {
        let error = CallError::new("uuid-456", "InternalError", "Internal server error");
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.starts_with("["));
        assert!(!json.starts_with("{"));
        assert_eq!(
            json,
            r#"[4,"uuid-456","InternalError","Internal server error",null]"#
        );
    }

    #[test]
    fn test_call_error_deserialization_array_format() {
        let json = r#"[4,"uuid-456","InternalError","Internal server error",{}]"#;
        let de: CallError = serde_json::from_str(json).unwrap();
        assert_eq!(de.message_type_id, 4);
        assert_eq!(de.unique_id, "uuid-456");
        assert_eq!(de.error_code, "InternalError");
        assert_eq!(de.error_description, "Internal server error");
    }

    #[test]
    fn test_call_error_deserialization_object_format() {
        let json =
            r#"{"4":4,"5":"uuid-456","6":"InternalError","7":"Internal server error","8":{}}"#;
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
