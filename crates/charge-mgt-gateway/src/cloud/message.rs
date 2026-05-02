use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessageInput {
    pub gateway_id: String,
    pub gateway_ip: String,
    pub vendor: String,
    pub charge_point_id: String,
    pub protocol: String,
    pub message_type: String,
    pub action: String,
    pub unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessage {
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
}

impl CloudMessage {
    pub fn new(input: CloudMessageInput, payload: serde_json::Value) -> Self {
        Self {
            gateway_id: input.gateway_id,
            gateway_ip: input.gateway_ip,
            vendor: input.vendor,
            charge_point_id: input.charge_point_id,
            protocol: input.protocol,
            message_type: input.message_type,
            action: input.action,
            unique_id: input.unique_id,
            payload,
            received_at: Utc::now(),
        }
    }

    pub fn topic(&self) -> String {
        self.vendor.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_message_new() {
        let input = CloudMessageInput {
            gateway_id: "gateway-01".to_string(),
            gateway_ip: "192.168.1.100".to_string(),
            vendor: "Alphas".to_string(),
            charge_point_id: "CB001".to_string(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "Call".to_string(),
            action: "BootNotification".to_string(),
            unique_id: "uuid-001".to_string(),
        };

        let msg = CloudMessage::new(input, serde_json::json!({"chargePointVendor": "Alphas"}));

        assert_eq!(msg.gateway_id, "gateway-01");
        assert_eq!(msg.vendor, "Alphas");
        assert_eq!(msg.topic(), "Alphas");
        assert_eq!(msg.message_type, "Call");
    }

    #[test]
    fn test_cloud_message_serialization() {
        let input = CloudMessageInput {
            gateway_id: "gateway-01".to_string(),
            gateway_ip: "192.168.1.100".to_string(),
            vendor: "Alphas".to_string(),
            charge_point_id: "CB001".to_string(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "Call".to_string(),
            action: "BootNotification".to_string(),
            unique_id: "uuid-001".to_string(),
        };

        let msg = CloudMessage::new(input, serde_json::json!({"chargePointVendor": "Alphas"}));

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"gateway_id\":\"gateway-01\""));
        assert!(json.contains("\"vendor\":\"Alphas\""));
        assert_eq!(msg.topic(), "Alphas");
    }
}
