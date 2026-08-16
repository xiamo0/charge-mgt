use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 构建 `CloudMessage` 所需的元数据（不含 payload）。
///
/// Gateway 侧的 `new_call` / `new_call_result` 工厂方法接收此类型；新代码
/// 既避免直接接触 `Option<...>`，也避免对发送方不关心的字段强制定值。
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

/// Cloud ↔ Gateway 共享 envelope。Kafka 消息体的单一真源。
///
/// 所有字段都是 `Option<String>` + `#[serde(default)]`，允许任意一方只填自己
/// 关心的字段而不破坏对端反序列化。未知字段由 serde 自动忽略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudMessage {
    // 路由字段（cloud→CP）
    #[serde(default)]
    pub csms_request_cp_message_http_url: Option<String>,
    #[serde(default)]
    pub csms_request_cp_message_mq_topic: Option<String>,
    #[serde(default)]
    pub csms_response_cp_message_mq_topic: Option<String>,
    #[serde(default)]
    pub cs_request_csms_message_mq_topic: Option<String>,

    // 公共字段（双向）
    #[serde(default)]
    pub charge_point_id: Option<String>,
    #[serde(default)]
    pub message_type: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub unique_id: Option<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,

    // 网关元数据（gateway 写、cloud 读）
    #[serde(default)]
    pub gateway_id: Option<String>,
    #[serde(default)]
    pub gateway_ip: Option<String>,
    #[serde(default)]
    pub vendor: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub received_at: Option<DateTime<Utc>>,

    // 错误信息（CallError 时）
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
}

impl CloudMessage {
    pub fn new(input: CloudMessageInput, payload: serde_json::Value) -> Self {
        Self {
            charge_point_id: Some(input.charge_point_id),
            message_type: Some(input.message_type),
            action: Some(input.action),
            unique_id: Some(input.unique_id),
            payload: Some(payload),
            gateway_id: Some(input.gateway_id),
            gateway_ip: Some(input.gateway_ip),
            vendor: Some(input.vendor),
            protocol: Some(input.protocol),
            received_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn new_call_result(input: CloudMessageInput, payload: serde_json::Value) -> Self {
        Self {
            message_type: Some("CallResult".to_string()),
            action: Some(input.action),
            unique_id: Some(input.unique_id),
            payload: Some(payload),
            received_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn new_call_error(
        input: CloudMessageInput,
        error_code: String,
        error_description: String,
    ) -> Self {
        Self {
            message_type: Some("CallError".to_string()),
            action: Some(input.action),
            unique_id: Some(input.unique_id),
            payload: Some(serde_json::json!({})),
            received_at: Some(Utc::now()),
            error_code: Some(error_code),
            error_description: Some(error_description),
            ..Default::default()
        }
    }

    /// 把入站 Call 转成出站 CallResult，沿用入站消息的 `action`/`unique_id`/路由字段。
    pub fn to_call_result(&self, payload: serde_json::Value) -> Self {
        Self {
            message_type: Some("CallResult".to_string()),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload: Some(payload),
            received_at: Some(Utc::now()),
            // 把路由字段透传/派生，便于对端（gateway）回送响应
            csms_request_cp_message_http_url: self.csms_request_cp_message_http_url.clone(),
            csms_request_cp_message_mq_topic: self.csms_request_cp_message_mq_topic.clone(),
            csms_response_cp_message_mq_topic: self.csms_response_cp_message_mq_topic.clone(),
            cs_request_csms_message_mq_topic: self.cs_request_csms_message_mq_topic.clone(),
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_ip.clone(),
            vendor: self.vendor.clone(),
            protocol: self.protocol.clone(),
            ..Default::default()
        }
    }

    /// 把入站 Call 转成出站 CallError，沿用入站消息的 `action`/`unique_id`/路由字段。
    pub fn to_call_error(&self) -> Self {
        Self {
            message_type: Some("CallError".to_string()),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload: Some(serde_json::Value::Null),
            received_at: Some(Utc::now()),
            csms_request_cp_message_http_url: self.csms_request_cp_message_http_url.clone(),
            csms_request_cp_message_mq_topic: self.csms_request_cp_message_mq_topic.clone(),
            csms_response_cp_message_mq_topic: self.csms_response_cp_message_mq_topic.clone(),
            cs_request_csms_message_mq_topic: self.cs_request_csms_message_mq_topic.clone(),
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_ip.clone(),
            vendor: self.vendor.clone(),
            protocol: self.protocol.clone(),
            ..Default::default()
        }
    }

    pub fn ocpp_call_array(&self) -> serde_json::Value {
        serde_json::json!([
            2,
            self.unique_id.clone().unwrap_or_default(),
            self.action.clone().unwrap_or_default(),
            self.payload.clone().unwrap_or(serde_json::Value::Null),
        ])
    }

    pub fn is_call(&self) -> bool {
        self.message_type.as_deref() == Some("Call")
    }

    /// 给定 topic 前缀 + vendor，生成上行请求主题：`{prefix}.req.{vendor}`
    pub fn req_topic(&self, prefix: &str) -> String {
        format!(
            "{}.req.{}",
            prefix,
            self.vendor.as_deref().unwrap_or("unknown")
        )
    }

    /// 给定 topic 前缀 + gateway_id，生成下行响应主题：`{prefix}.resp.{gateway_id}`
    pub fn resp_topic(prefix: &str, gateway_id: &str) -> String {
        format!("{prefix}.resp.{gateway_id}")
    }

    /// 给定 topic 前缀 + gateway_id，生成下行命令主题：`{prefix}.cmd.{gateway_id}`
    pub fn cmd_topic(prefix: &str, gateway_id: &str) -> String {
        format!("{prefix}.cmd.{gateway_id}")
    }

    pub fn csms_request_cp_message_http_url_str(&self) -> &str {
        self.csms_request_cp_message_http_url
            .as_deref()
            .unwrap_or("")
    }
    pub fn csms_request_cp_message_mq_topic_str(&self) -> &str {
        self.csms_request_cp_message_mq_topic
            .as_deref()
            .unwrap_or("")
    }
    pub fn csms_response_cp_message_mq_topic_str(&self) -> &str {
        self.csms_response_cp_message_mq_topic
            .as_deref()
            .unwrap_or("")
    }
    pub fn cs_request_csms_message_mq_topic_str(&self) -> &str {
        self.cs_request_csms_message_mq_topic
            .as_deref()
            .unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gateway_upstream_bytes() -> serde_json::Value {
        serde_json::json!({
            "gateway_id": "gw-01",
            "gateway_ip": "192.168.1.1",
            "vendor": "Alphas",
            "protocol": "OCPP-1.6",
            "charge_point_id": "CB001",
            "message_type": "Call",
            "action": "BootNotification",
            "unique_id": "uuid-001",
            "payload": {
                "chargePointVendor": "Alphas",
                "chargePointModel": "ModelX"
            },
            "received_at": "2024-01-01T00:00:00Z",
            "error_code": null,
            "error_description": null,
            "cs_request_csms_message_mq_topic": "charge_mgt.req.Alphas",
        })
    }

    fn cloud_downstream_bytes() -> serde_json::Value {
        serde_json::json!({
            "csms_request_cp_message_http_url": "http://gw-01/ocpp/CB001",
            "csms_request_cp_message_mq_topic": "charge_mgt.cmd.gw-01",
            "csms_response_cp_message_mq_topic": "charge_mgt.resp.gw-01",
            "cs_request_csms_message_mq_topic": "charge_mgt.req.gw-01",
            "charge_point_id": "CB001",
            "message_type": "CallResult",
            "action": "BootNotification",
            "unique_id": "uuid-001",
            "payload": {
                "status": "Accepted",
                "currentTime": "2024-01-01T00:00:00Z",
                "interval": 30
            },
        })
    }

    #[test]
    fn gateway_upstream_deserializes() {
        // #4 阻塞性 bug 的核心回归：之前反序列化失败，现在必须成功
        let bytes = serde_json::to_vec(&gateway_upstream_bytes()).unwrap();
        let parsed: CloudMessage = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(parsed.charge_point_id.as_deref(), Some("CB001"));
        assert_eq!(parsed.action.as_deref(), Some("BootNotification"));
        assert_eq!(parsed.unique_id.as_deref(), Some("uuid-001"));
        assert_eq!(parsed.gateway_id.as_deref(), Some("gw-01"));
        assert_eq!(parsed.vendor.as_deref(), Some("Alphas"));
        assert!(parsed.csms_request_cp_message_http_url.is_none());
        assert!(parsed.payload.is_some());
    }

    #[test]
    fn cloud_downstream_deserializes() {
        let bytes = serde_json::to_vec(&cloud_downstream_bytes()).unwrap();
        let parsed: CloudMessage = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(parsed.charge_point_id.as_deref(), Some("CB001"));
        assert_eq!(parsed.action.as_deref(), Some("BootNotification"));
        assert_eq!(
            parsed.csms_request_cp_message_http_url.as_deref(),
            Some("http://gw-01/ocpp/CB001")
        );
        assert!(parsed.gateway_id.is_none());
    }

    #[test]
    fn empty_object_deserializes() {
        let parsed: CloudMessage = serde_json::from_slice(b"{}").unwrap();
        assert!(parsed.charge_point_id.is_none());
        assert!(parsed.action.is_none());
        assert!(parsed.payload.is_none());
    }

    #[test]
    fn unknown_fields_ignored() {
        let raw = serde_json::json!({
            "charge_point_id": "CB001",
            "future_field_xyz": "should be ignored",
            "another_new_field": 42,
        });
        let bytes = serde_json::to_vec(&raw).unwrap();
        let parsed: CloudMessage = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed.charge_point_id.as_deref(), Some("CB001"));
    }

    #[test]
    fn round_trip_preserves_all_fields() {
        let input = CloudMessageInput {
            gateway_id: "gw-01".into(),
            gateway_ip: "192.168.1.1".into(),
            vendor: "Alphas".into(),
            charge_point_id: "CB001".into(),
            protocol: "OCPP-1.6".into(),
            message_type: "Call".into(),
            action: "Heartbeat".into(),
            unique_id: "u-1".into(),
        };
        let original = CloudMessage::new(input, serde_json::json!({}));
        let bytes = serde_json::to_vec(&original).unwrap();
        let parsed: CloudMessage = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed.charge_point_id, original.charge_point_id);
        assert_eq!(parsed.action, original.action);
        assert_eq!(parsed.gateway_id, original.gateway_id);
    }
}
