use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessage {
    pub http_url: String,
    pub gateway_id: String,
    pub gateway_ip: String,
    pub vendor: String,
    pub charge_point_id: String,
    pub protocol: String,
    pub message_type: String,
    pub action: String,
    pub unique_id: String,
    pub payload: serde_json::Value,
    pub received_at: DateTime<Utc>,
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
}

impl CloudMessage {
    pub fn new_call_result(&self, payload: serde_json::Value) -> Self {
        Self {
            http_url: self.http_url.clone(),
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_ip.clone(),
            vendor: self.vendor.clone(),
            charge_point_id: self.charge_point_id.clone(),
            protocol: self.protocol.clone(),
            message_type: "CallResult".to_string(),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload,
            received_at: Utc::now(),
            error_code: None,
            error_description: None,
        }
    }

    pub fn new_call_error(&self, error_code: &str, error_description: &str) -> Self {
        Self {
            http_url: self.http_url.clone(),
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_ip.clone(),
            vendor: self.vendor.clone(),
            charge_point_id: self.charge_point_id.clone(),
            protocol: self.protocol.clone(),
            message_type: "CallError".to_string(),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload: serde_json::Value::Null,
            received_at: Utc::now(),
            error_code: Some(error_code.to_string()),
            error_description: Some(error_description.to_string()),
        }
    }

    pub fn is_call(&self) -> bool {
        self.message_type == "Call"
    }
}
