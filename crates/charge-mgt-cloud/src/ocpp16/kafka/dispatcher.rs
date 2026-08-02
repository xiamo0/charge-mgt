use crate::ocpp16::entity::sent_messages;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_from_cp_handler::{Handler, UnknownRequest};
use crate::state::AppState;
use chrono::Local;
use ocpp_1_6::calls::{
    AuthorizeRequest, BootNotificationRequest, DataTransferRequest, HeartbeatRequest,
    MeterValuesRequest, StartTransactionRequest, StatusNotificationRequest, StopTransactionRequest,
};
use ocpp_1_6::protocol::{
    ACTION_AUTHORIZE, ACTION_BOOT_NOTIFICATION, ACTION_DATA_TRANSFER, ACTION_HEARTBEAT,
    ACTION_METER_VALUES, ACTION_START_TRANSACTION, ACTION_STATUS_NOTIFICATION,
    ACTION_STOP_TRANSACTION,
};
use sea_orm::{EntityTrait, Set, TryInsertResult};
use tracing::{info, log, warn};
use crate::error::AppError;

pub struct MessageDispatcher {
    state: AppState,
}

impl MessageDispatcher {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn dispatch(&self, bytes: &[u8]) -> Result<(), AppError> {
        let msg: CloudMessage =
            serde_json::from_slice(bytes).map_err(|e| AppError::OCPP_1_6_ERROR{
                action:"mq".to_string(),
                detail:e.to_string()
            })?;

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

        if let Some(db) = &self.state.db {
            let res = sent_messages::Entity::insert(new_message)
                .on_conflict_do_nothing()
                .exec(db)
                .await
                .map_err(|e| AppError::OCPP_1_6_ERROR {
                    action: "mq".to_string(),
                    detail: e.to_string()
                })?;

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
        }

        let handler_result = Self::route_handler(&self.state, &msg).await;

        let response = match handler_result {
            Ok(payload) => msg.new_call_result(payload),
            Err(e) => {
               msg.new_call_error("InternalError", &format!("处理消息失败: {}", e))
            }
        };

        let resp_bytes =
            serde_json::to_vec(&response).map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "mq".to_string(),
                detail: e.to_string()
            })?;

        let topic = self.resp_topic(&msg.gateway_id);

        if let Ok(pr) = self.state.producer() {
            pr.send_resp(&topic, &msg.unique_id, &resp_bytes)
                .await
                .map_err(|e| AppError::OCPP_1_6_ERROR {
                    action: "mq".to_string(),
                    detail: e.to_string()
                })?;
        } else {
            log::error!("Kafka producer 未初始化，无法发送响应消息");
        }
        /* if let Some(producer) = &self.state.producer {
            producer
                .send_resp(&topic, &msg.unique_id, &resp_bytes)
                .await
                .map_err(DispatchError::Kafka)?;
        } else {
            log::error!("Kafka producer 未初始化，无法发送响应消息");
        }*/

        Ok(())
    }

    async fn route_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<serde_json::Value, AppError> {
        match msg.action.as_str() {
            ACTION_BOOT_NOTIFICATION => BootNotificationRequest::handle(state, msg).await,
            ACTION_HEARTBEAT => HeartbeatRequest::handle(state, msg).await,
            ACTION_AUTHORIZE => AuthorizeRequest::handle(state, msg).await,
            ACTION_START_TRANSACTION => StartTransactionRequest::handle(state, msg).await,
            ACTION_STOP_TRANSACTION => StopTransactionRequest::handle(state, msg).await,
            ACTION_METER_VALUES => MeterValuesRequest::handle(state, msg).await,
            ACTION_STATUS_NOTIFICATION => StatusNotificationRequest::handle(state, msg).await,
            ACTION_DATA_TRANSFER => DataTransferRequest::handle(state, msg).await,

            _other => UnknownRequest::handle(state, msg).await,
        }
    }

    fn resp_topic(&self, gateway_id: &str) -> String {
        if let Some(config) = &self.state.config {
            format!("{}.resp.{}", config.kafka.topic_prefix, gateway_id)
        } else {
            format!("{}.resp.{}", "unknown_topic_prefix", gateway_id)
        }
    }
}
