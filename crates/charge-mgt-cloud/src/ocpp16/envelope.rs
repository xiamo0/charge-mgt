use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMessage {
    pub csms_request_cp_message_http_url: String,
    pub csms_request_cp_message_mq_topic: String,
    pub csms_response_cp_message_mq_topic: String,
    pub cs_request_csms_message_mq_topic: String,
    pub charge_point_id: String,
    pub message_type: String,
    pub action: String,
    pub unique_id: String,
    pub payload: serde_json::Value,
}

impl CloudMessage {
    pub fn new_call_result(&self, payload: serde_json::Value) -> Self {
        Self {
            
            csms_request_cp_message_http_url: self.csms_request_cp_message_http_url.clone(),
            charge_point_id: self.charge_point_id.clone(),
            cs_request_csms_message_mq_topic: self.cs_request_csms_message_mq_topic.clone(),
            csms_request_cp_message_mq_topic: self.csms_request_cp_message_mq_topic.clone(),
            csms_response_cp_message_mq_topic: self.csms_response_cp_message_mq_topic.clone(),

            message_type: "CallResult".to_string(),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload,
        }
    }

    pub fn new_call_error(&self) -> Self {
        Self {
            csms_request_cp_message_http_url: self.csms_request_cp_message_http_url.clone(),
            charge_point_id: self.charge_point_id.clone(),
            cs_request_csms_message_mq_topic: self.cs_request_csms_message_mq_topic.clone(),
            csms_request_cp_message_mq_topic: self.csms_request_cp_message_mq_topic.clone(),
            csms_response_cp_message_mq_topic: self.csms_response_cp_message_mq_topic.clone(),

            message_type: "CallError".to_string(),
            action: self.action.clone(),
            unique_id: self.unique_id.clone(),
            payload: serde_json::Value::Null,
        }
    }

    pub fn is_call(&self) -> bool {
        self.message_type == "Call"
    }
}
