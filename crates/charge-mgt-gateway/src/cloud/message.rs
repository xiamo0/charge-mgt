//! 云端消息格式定义
//!
//! 定义网关与云端之间通过 Kafka 传递的统一消息结构及主题命名规则。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 构建 CloudMessage 所需的元数据（不含 payload）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessageInput {
    /// 网关 ID
    pub gateway_id: String,
    /// 网关 IP 地址
    pub gateway_ip: String,
    /// 充电桩厂商名称
    pub vendor: String,
    /// 充电桩 ID
    pub charge_point_id: String,
    /// 协议版本（如 `OCPP-1.6`）
    pub protocol: String,
    /// OCPP 消息类型（Call / CallResult / CallError）
    pub message_type: String,
    /// OCPP action 名称
    pub action: String,
    /// OCPP 消息唯一 ID
    pub unique_id: String,
}

/// 发往/来自云端的完整消息，序列化后作为 Kafka 消息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessage {
    /// 网关 ID
    pub gateway_id: String,
    /// 网关 IP 地址
    pub gateway_ip: String,
    /// 充电桩厂商名称
    pub vendor: String,
    /// 充电桩 ID
    pub charge_point_id: String,
    /// 协议版本（如 `OCPP-1.6`）
    pub protocol: String,
    /// OCPP 消息类型（Call / CallResult / CallError）
    pub message_type: String,
    /// OCPP action 名称
    pub action: String,
    /// OCPP 消息唯一 ID
    pub unique_id: String,
    /// 消息体 JSON
    pub payload: serde_json::Value,
    /// 网关接收消息的时间戳
    pub received_at: DateTime<Utc>,
    /// 错误码（CallError 时有值）
    pub error_code: Option<String>,
    /// 错误描述（CallError 时有值）
    pub error_description: Option<String>,
}

impl CloudMessage {
    /// 创建 OCPP Call 类型的上行消息
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
            error_code: None,
            error_description: None,
        }
    }

    /// 创建 CallResult 类型的下行响应消息
    pub fn new_call_result(input: CloudMessageInput, payload: serde_json::Value) -> Self {
        Self {
            gateway_id: input.gateway_id,
            gateway_ip: input.gateway_ip,
            vendor: input.vendor,
            charge_point_id: input.charge_point_id,
            protocol: input.protocol,
            message_type: "CallResult".to_string(),
            action: input.action,
            unique_id: input.unique_id,
            payload,
            received_at: Utc::now(),
            error_code: None,
            error_description: None,
        }
    }

    /// 创建 CallError 类型的下行错误响应消息
    pub fn new_call_error(
        input: CloudMessageInput,
        error_code: String,
        error_description: String,
    ) -> Self {
        Self {
            gateway_id: input.gateway_id,
            gateway_ip: input.gateway_ip,
            vendor: input.vendor,
            charge_point_id: input.charge_point_id,
            protocol: input.protocol,
            message_type: "CallError".to_string(),
            action: input.action,
            unique_id: input.unique_id,
            payload: serde_json::json!({}),
            received_at: Utc::now(),
            error_code: Some(error_code),
            error_description: Some(error_description),
        }
    }

    /// 上行请求主题：`{prefix}.{suffix}.{vendor}`
    pub fn req_topic(&self, prefix: &str, suffix: &str) -> String {
        format!("{}.{}.{}", prefix, suffix, self.vendor)
    }

    /// 响应主题：`{prefix}.{suffix}.{gateway_id}`（Kafka 响应通道模式）
    pub fn resp_topic(prefix: &str, suffix: &str, gateway_id: &str) -> String {
        format!("{}.{}.{}", prefix, suffix, gateway_id)
    }

    /// 下行命令主题：`{prefix}.{suffix}.{gateway_id}`（Redis 响应通道模式）
    pub fn cmd_topic(prefix: &str, suffix: &str, gateway_id: &str) -> String {
        format!("{}.{}.{}", prefix, suffix, gateway_id)
    }

    /// 返回厂商名称，用作 Kafka 消息分区键
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
        assert!(msg.error_code.is_none());
    }

    #[test]
    fn test_cloud_message_call_result() {
        let input = CloudMessageInput {
            gateway_id: "gateway-01".to_string(),
            gateway_ip: "192.168.1.100".to_string(),
            vendor: "Alphas".to_string(),
            charge_point_id: "CB001".to_string(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "CallResult".to_string(),
            action: "BootNotification".to_string(),
            unique_id: "uuid-001".to_string(),
        };

        let msg = CloudMessage::new_call_result(
            input,
            serde_json::json!({"status": "Accepted", "interval": 30}),
        );

        assert_eq!(msg.message_type, "CallResult");
        assert_eq!(msg.action, "BootNotification");
        assert!(msg.error_code.is_none());
    }

    #[test]
    fn test_cloud_message_call_error() {
        let input = CloudMessageInput {
            gateway_id: "gateway-01".to_string(),
            gateway_ip: "192.168.1.100".to_string(),
            vendor: "Alphas".to_string(),
            charge_point_id: "CB001".to_string(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "CallError".to_string(),
            action: "BootNotification".to_string(),
            unique_id: "uuid-001".to_string(),
        };

        let msg = CloudMessage::new_call_error(
            input,
            "SecurityError".to_string(),
            "Unauthorized charge point".to_string(),
        );

        assert_eq!(msg.message_type, "CallError");
        assert_eq!(msg.error_code, Some("SecurityError".to_string()));
        assert_eq!(
            msg.error_description,
            Some("Unauthorized charge point".to_string())
        );
    }

    #[test]
    fn test_cloud_message_topic_methods() {
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

        let msg = CloudMessage::new(input, serde_json::json!({}));

        assert_eq!(msg.req_topic("charge_mgt", "req"), "charge_mgt.req.Alphas");
        assert_eq!(
            CloudMessage::resp_topic("charge_mgt", "resp", "gateway-01"),
            "charge_mgt.resp.gateway-01"
        );
        assert_eq!(
            CloudMessage::cmd_topic("charge_mgt", "cmd", "gateway-01"),
            "charge_mgt.cmd.gateway-01"
        );
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
        assert!(json.contains("\"error_code\":null"));
        assert!(json.contains("\"error_description\":null"));
        assert_eq!(msg.topic(), "Alphas");
    }
}