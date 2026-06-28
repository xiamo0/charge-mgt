use crate::entity::sent_messages;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::ocpp16::cp_request_handlers::{UnkonwnRequest, authorize, boot_notification, data_transfer,Handler, heartbeat,meter_values, start_transaction,status_notification,stop_transaction};
use crate::state::AppState;
use chrono::Local;
use ocpp_1_6::calls::{AuthorizeRequest, BootNotificationRequest, HeartbeatRequest, StartTransactionRequest, StopTransactionRequest, MeterValuesRequest, StatusNotificationRequest, DataTransferRequest};
use sea_orm::sea_query::Value;
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, Set, Statement, TryInsertResult};
use tracing::{info, warn};
use ocpp_1_6::protocol::{ACTION_AUTHORIZE, ACTION_BOOT_NOTIFICATION, ACTION_HEARTBEAT,ACTION_START_TRANSACTION,ACTION_STOP_TRANSACTION,ACTION_METER_VALUES,ACTION_STATUS_NOTIFICATION,ACTION_DATA_TRANSFER};

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

        let new_message = sent_messages::ActiveModel {
            unique_id: Set(msg.unique_id.clone()),
            gateway_id: Set(msg.gateway_id.clone()),
            charge_point_id: Set(msg.charge_point_id.clone()),
            direction: Set(msg.message_type.clone()),
            action: Set(msg.action.clone()),
            message_type: Set(msg.message_type.clone()),
            received_at: Set(Local::now().with_timezone(Local::now().offset())),
            processed_at: Set(Local::now().with_timezone(Local::now().offset())),
        };

        let res = sent_messages::Entity::insert(new_message)
            .on_conflict_do_nothing()
            .exec(&self.state.db)
            .await
            .map_err(|e| DispatchError::Database(e.to_string()))?;

        match res {
            TryInsertResult::Inserted(keys) => {
                info!("新消息，开始处理{:?}", keys.last_insert_id);
            }
            TryInsertResult::Conflicted => {
                info!(
                unique_id = %msg.unique_id,
                "重复消息，跳过（幂等保护）"
                );
                return Ok(());
            }
            TryInsertResult::Empty => {
                info!(
                unique_id = %msg.unique_id,
                "消息已存在但未插入，可能是数据库异常，跳过"
                );
                return Ok(());
            }
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
            ACTION_BOOT_NOTIFICATION => BootNotificationRequest::handle(state, msg).await,
            ACTION_HEARTBEAT => HeartbeatRequest::handle(state, msg).await,
            ACTION_AUTHORIZE => AuthorizeRequest::handle(state, msg).await,
            ACTION_START_TRANSACTION => StartTransactionRequest::handle(state, msg).await,
            ACTION_STOP_TRANSACTION=> StopTransactionRequest::handle(state, msg).await,
            ACTION_METER_VALUES => MeterValuesRequest::handle(state, msg).await,
            ACTION_STATUS_NOTIFICATION=> StatusNotificationRequest::handle(state, msg).await,
            ACTION_DATA_TRANSFER => DataTransferRequest::handle(state, msg).await,

                other => UnkonwnRequest::handle(state, msg).await,
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
