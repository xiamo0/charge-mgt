use crate::error::AppError;
use crate::ocpp16::entity::sent_messages;
use crate::ocpp16::message_from_cp_handler::{Handler, UnknownRequest};
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
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
use tracing::{info, warn};

pub struct MessageDispatcher {
    state: AppState,
}

impl MessageDispatcher {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn dispatch(&self, bytes: &[u8]) -> Result<(), AppError> {
        let msg: CloudMessage =
            serde_json::from_slice(bytes).map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "mq".to_string(),
                detail: format!("反序列化 CloudMessage 失败: {e}"),
            })?;

        // 幂等屏障：先 INSERT 唯一键（unique_id），若已存在则跳过本次处理。
        // DB 必须可用；不可用则返回错误，不绕过幂等保护（否则重复消息会被多次处理）。
        let db = self.state.db()?;
        let now = Local::now().with_timezone(Local::now().offset());
        let new_message = sent_messages::ActiveModel {
            unique_id: Set(msg.unique_id.clone().unwrap_or_default()),
            charge_point_id: Set(msg.charge_point_id.clone().unwrap_or_default()),
            direction: Set(msg.message_type.clone().unwrap_or_default()),
            action: Set(msg.action.clone().unwrap_or_default()),
            message_type: Set(msg.message_type.clone().unwrap_or_default()),
            received_at: Set(now),
            processed_at: Set(now),
        };

        let res = sent_messages::Entity::insert(new_message)
            .on_conflict_do_nothing()
            .exec(db)
            .await
            .map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "mq".to_string(),
                detail: format!("幂等写入失败: {e}"),
            })?;

        match res {
            TryInsertResult::Inserted(_) => {
                info!(unique_id = %msg.unique_id.as_deref().unwrap_or(""), "新消息，开始处理");
            }
            TryInsertResult::Conflicted => {
                info!(
                    unique_id = %msg.unique_id.as_deref().unwrap_or(""),
                    "重复消息，跳过（幂等保护）"
                );
                return Ok(());
            }
            TryInsertResult::Empty => {
                warn!(
                    unique_id = %msg.unique_id.as_deref().unwrap_or(""),
                    "消息已存在但未插入，可能是数据库异常，跳过"
                );
                return Ok(());
            }
        }

        let handler_result = Self::route_handler(&self.state, &msg).await;

        let response = match handler_result {
            Ok(payload) => msg.to_call_result(payload),
            Err(_) => msg.to_call_error(),
        };

        let resp_bytes = serde_json::to_vec(&response).map_err(|e| AppError::OCPP_1_6_ERROR {
            action: "mq".to_string(),
            detail: format!("序列化响应失败: {e}"),
        })?;

        // Kafka producer 不可用 → 直接返回错误；让 Kafka offset 不提交，
        // 下次重试该消息，避免桩侧响应永远丢失。
        let pr = self.state.producer()?;
        let topic = &msg
            .csms_response_cp_message_mq_topic
            .as_deref()
            .unwrap_or("");
        pr.send_resp(topic, &msg.unique_id.as_deref().unwrap_or(""), &resp_bytes)
            .await
            .map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "mq".to_string(),
                detail: format!("发送响应到 Kafka 失败: {e}"),
            })?;

        Ok(())
    }

    async fn route_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<serde_json::Value, AppError> {
        match msg.action.as_deref().unwrap_or("") {
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
}
