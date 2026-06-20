use sea_orm::sea_query::Value;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use tracing::{info, warn};

use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::ocpp::handlers::{boot_notification, heartbeat};
use crate::state::AppState;
use crate::entity::sent_messages::{SentMessages, SentMessagesActiveModel};

pub struct MessageDispatcher {
    state: AppState,
}

impl MessageDispatcher {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn dispatch(&self, bytes: &[u8]) -> Result<(), DispatchError> {
        let msg: CloudMessage =
            serde_json::from_slice(bytes).map_err(|e| DispatchError::Malformed(e.to_string()))?;


        let new_message = SentMessages::ActiveModel {
            unique_id: msg.unique_id.clone(),
            gateway_id: msg.gateway_id.clone(),
            charge_point_id: msg.charge_point_id.clone(),
            direction: msg.message_type.clone(),
            action: msg.action.clone(),
            message_type: msg.message_type.clone(),
            received_at: msg.received_at,
            processed_at: chrono::Utc::now(),
        };
        let result = SentMessages::insert(new_message)
            .on_conflict(
                OnConflict::columns([SentMessages::Column::UniqueId])
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&self.state.db)
            .await?;

        let exists = SentMessages::find_by_id(msg.unique_id.clone()).one(&self.state.db).await.is_some()?;
        if exists {
            info!(
                unique_id = %msg.unique_id,
                "重复消息，跳过（幂等保护）"
            );
            return Ok(());
        }
        let handler_result = Self::route_handler(&self.state, &msg).await;

        let response = match handler_result {
            Ok(payload) => msg.new_call_result(payload),
            Err(e) => {
                let (code, desc) = e.to_ocpp_error();
                warn!(
                    action = %msg.action,
                    unique_id = %msg.unique_id,
                    error_code = %code,
                    "handler 返回 CallError"
                );
                msg.new_call_error(code, &desc)
            }
        };

        let resp_bytes =
            serde_json::to_vec(&response).map_err(|e| DispatchError::Serialize(e.to_string()))?;

        let topic = self.resp_topic(&msg.gateway_id);
        self.state
            .producer
            .send_resp(&topic, &msg.unique_id, &resp_bytes)
            .await
            .map_err(DispatchError::Kafka)?;

        Ok(())
    }

    async fn route_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<serde_json::Value, HandlerError> {
        match msg.action.as_str() {
            "BootNotification" => boot_notification::handle(state, msg).await,
            "Heartbeat" => heartbeat::handle(state, msg).await,
            other => {
                warn!(action = %other, "不支持的 OCPP action");
                Err(HandlerError::NotSupported(format!(
                    "action '{other}' not implemented"
                )))
            }
        }
    }

    fn resp_topic(&self, gateway_id: &str) -> String {
        format!(
            "{}.resp.{}",
            self.state.config.kafka.topic_prefix, gateway_id
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("消息格式错误：{0}")]
    Malformed(String),
    #[error("数据库错误：{0}")]
    Database(String),
    #[error("序列化错误：{0}")]
    Serialize(String),
    #[error("Kafka 发送错误：{0}")]
    Kafka(String),
}
