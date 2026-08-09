use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::{ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION, CALL, CALLERROR, CALLRESULT};

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::ClearChargingProfileRequest;
use ocpp_1_6::confs::ClearChargingProfileConfirmation;

impl Handler<ClearChargingProfileConfirmation> for ClearChargingProfileRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ClearChargingProfileConfirmation, AppError> {
        let http_sender = state.http_sender().map_err(|e| AppError::OCPP_1_6_ERROR {
            action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
            detail: format!("请求ID {} ,获取 HTTP sender 失败: {e}", msg.unique_id),
        })?;

        let ocpp_call = serde_json::json!([CALL, &msg.unique_id, &msg.action, &msg.payload,]);

        let resp_value = http_sender
            .post_ocpp(&msg.csms_request_cp_message_http_url, &ocpp_call)
            .await
            .map_err(|e| AppError::OCPP_1_6_ERROR {
                action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                detail: format!("请求ID {} ,HTTP 请求失败: {e}", msg.unique_id),
            })?;

        let arr = resp_value
            .as_array()
            .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                detail: format!(
                    "请求ID {} ,OCPP 响应不是有效数组: {resp_value}",
                    msg.unique_id
                ),
            })?;

        let msg_type = arr.first().and_then(|v| v.as_i64());
        match msg_type {
            Some(CALLRESULT) => {
                //     todo!()
                // ✅ 从引用反序列化，避免 clone payload
                let payload = arr.get(2).ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {} ,CALLRESULT 缺少 payload 字段: {resp_value}",
                        msg.unique_id
                    ),
                })?;

                // ✅ serde_json::from_value 接受 &Value，无需 clone
                let result = serde_json::from_value(payload.clone()).map_err(|e| AppError::OCPP_1_6_ERROR {
                    action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {} ,反序列化 CancelReservationConfirmation 失败: {e}, raw={payload}",
                        msg.unique_id
                    ),
                });
                result
            }
            Some(CALLERROR) => {
                let error_code = arr.get(2).and_then(|v| v.as_str()).unwrap_or("Unknown");
                let error_desc = arr.get(3).and_then(|v| v.as_str()).unwrap_or("");
                return Err(AppError::OCPP_1_6_ERROR {
                    action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {} ,OCPP CALLERROR: code={error_code}, description={error_desc}",
                        msg.unique_id
                    ),
                });
            }
            _ => {
                return Err(AppError::OCPP_1_6_ERROR {
                    action: ACTION_CLEAR_CHARGING_PROFILE_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {} ,未知的 OCPP 消息类型: {:?}",
                        msg.unique_id, msg_type
                    ),
                });
            }
        }
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<ClearChargingProfileConfirmation, AppError> {
        todo!()
    }
}
