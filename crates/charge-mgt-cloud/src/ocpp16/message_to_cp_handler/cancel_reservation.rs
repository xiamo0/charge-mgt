use std::time::Duration;

use crate::error::AppError;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::ACTION_CANCEL_RESERVATION_CONFIRMATION;
use ocpp_1_6::{CALL, CALLERROR, CALLRESULT};
use ocpp_1_6::calls::CancelReservationRequest;
use ocpp_1_6::confs::CancelReservationConfirmation;

/// MQ 模式下等待桩响应的默认超时。
const MQ_RESP_TIMEOUT: Duration = Duration::from_secs(10);

impl Handler<CancelReservationConfirmation> for CancelReservationRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, AppError> {
        let http_sender = state.http_sender()?;
        let unique_id = msg.unique_id.clone();
        let ocpp_call = serde_json::json!([CALL, unique_id.as_str(), msg.action, msg.payload,]);

        let resp_value = http_sender.post_ocpp(&msg.http_url, &ocpp_call).await?;

        let payload = resp_value
            .as_array()
            .and_then(|arr| arr.get(2))
            .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                detail: format!("请求ID {unique_id} ,OCPP 响应不是有效数组: {resp_value}"),
            })?
            .clone();

        let result = serde_json::from_value(payload).map_err(|e| AppError::OCPP_1_6_ERROR {
            action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
            detail: format!("请求ID {unique_id} ,反序列化 CancelReservationConfirmation 失败: {e}"),
        });

        match result {
            Ok(result) => {
                return Ok(result);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, AppError> {
        let config = state.config()?;
        let producer = state.producer()?;
        let mq = state.mq_dispatcher()?;

        let unique_id = msg.unique_id.clone();
        let ocpp_call = serde_json::json!([CALL, unique_id.as_str(), msg.action, msg.payload,]);
        let call_bytes = serde_json::to_vec(&ocpp_call).map_err(|e| AppError::OCPP_1_6_ERROR {
            action: "mq".to_string(),
            detail: e.to_string(),
        })?;

        let req_topic = format!("{}.req.{}", config.kafka.topic_prefix, msg.gateway_id);
        producer
            .send_call(&req_topic, &unique_id, &call_bytes)
            .await
            .map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "mq".to_string(),
                detail: e.to_string(),
            })?;

        let resp = mq.await_response(&unique_id, MQ_RESP_TIMEOUT).await?;

        let arr: Vec<serde_json::Value> =
            serde_json::from_slice(&resp.bytes).map_err(|e| AppError::OCPP_1_6_ERROR {
                action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                detail: format!("MQ 响应 JSON 解析失败: {e}"),
            })?;

        match arr.first().and_then(|v| v.as_i64()) {
            Some(msg_type) if msg_type == i64::from(CALLRESULT) => {
                let conf_payload = arr.get(2).cloned().ok_or_else(|| {
                    AppError::OCPP_1_6_ERROR {
                        action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                        detail: "CALLRESULT 缺少 payload".into(),
                    }
                })?;
                serde_json::from_value(conf_payload).map_err(|e| AppError::OCPP_1_6_ERROR {
                    action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                    detail: format!("反序列化 CancelReservationConfirmation 失败: {e}"),
                })
            }
            Some(msg_type) if msg_type == i64::from(CALLERROR) => {
                let code = arr.get(2).and_then(|v| v.as_str()).unwrap_or("Unknown");
                let desc = arr.get(3).and_then(|v| v.as_str()).unwrap_or("");
                Err(AppError::OCPP_1_6_ERROR {
                    action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                    detail: format!("桩返回 CALLERROR [{code}]: {desc}"),
                })
            }
            other => Err(AppError::OCPP_1_6_ERROR {
                action: ACTION_CANCEL_RESERVATION_CONFIRMATION.into(),
                detail: format!("未知 MQ 响应类型: {other:?}"),
            }),
        }
    }
}
