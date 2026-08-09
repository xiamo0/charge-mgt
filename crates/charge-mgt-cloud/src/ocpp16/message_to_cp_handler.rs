use std::time::Duration;

use ocpp_1_6::{CALL, CALLERROR, CALLRESULT};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::ocpp16::envelope::CloudMessage;
use crate::state::AppState;

// 云平台向充电桩发送请求
pub mod cancel_reservation;
pub mod change_availability;
pub mod change_configuration;
pub mod clear_cache;
pub mod clear_charging_profile;
pub mod data_transfer;
pub mod get_composite_schedule;
pub mod get_configuration;
pub mod get_diagnostics;
pub mod get_local_list_version;
pub mod remote_start_transaction;
pub mod remote_stop_transaction;
pub mod reserve_now;
pub mod send_local_list;
pub mod set_charging_profile;
pub mod trigger_message;
pub mod unlock_connector;
pub mod update_firmware;

/// MQ 出站模式默认等待桩响应超时。
pub const MQ_RESP_TIMEOUT: Duration = Duration::from_secs(10);

pub trait Handler<T: Serialize> {
    async fn handle(state: &AppState, msg: &CloudMessage) -> Result<serde_json::Value, AppError> {
        let r = Self::handle_detail(state, msg).await?;

        Ok(serde_json::to_value(&r)?)
    }

    async fn handle_detail(state: &AppState, msg: &CloudMessage) -> Result<T, AppError>;
}

/// MQ 出站通用流程：组装 CALL → 发到 req topic → 等 resp topic 回 CALLRESULT → 反序列化为 `T`。
///
/// 协议细节：
/// - 出站包络：`[2, unique_id, action, payload]`
/// - 入站包络：`[3, unique_id, payload]`（成功）或 `[4, unique_id, code, desc, detail]`（失败）
///
/// `action_label` 仅用于错误信息中的可读字段，不参与协议。
#[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
pub async fn dispatch_mq_call<T>(
    state: &AppState,
    msg: &CloudMessage,
    action_label: &str,
) -> Result<T, AppError>
where
    T: for<'de> Deserialize<'de>,
{
    let producer = state.producer()?;
    let mq = state.mq_dispatcher()?;

    let ocpp_call = serde_json::json!([CALL, &msg.unique_id, msg.action, msg.payload]);
    let call_bytes = serde_json::to_vec(&ocpp_call).map_err(|e| AppError::OCPP_1_6_ERROR {
        action: action_label.to_string(),
        detail: format!("请求ID {}, 序列化 CALL 失败：{e}", msg.unique_id),
    })?;

    producer
        .send_call(&msg.csms_request_cp_message_mq_topic, &msg.unique_id, &call_bytes)
        .await
        .map_err(|e| AppError::OCPP_1_6_ERROR {
            action: action_label.to_string(),
            detail: format!("请求ID {}, 发送 CALL 失败：{e}", msg.unique_id),
        })?;

    let resp = mq.await_response(&msg.unique_id, MQ_RESP_TIMEOUT).await?;
    let arr: Vec<serde_json::Value> =
        serde_json::from_slice(&resp.bytes).map_err(|e| AppError::OCPP_1_6_ERROR {
            action: action_label.to_string(),
            detail: format!("请求ID {}, MQ 响应 JSON 解析失败：{e}", msg.unique_id),
        })?;

    match arr.first().and_then(|v| v.as_i64()) {
        Some(t) if t == CALLRESULT => {
            let payload = arr.get(2).ok_or_else(|| AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!("请求ID {}, CALLRESULT 缺少 payload", msg.unique_id),
            })?;
            serde_json::from_value(payload.clone()).map_err(|e| AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!(
                    "请求ID {}, 反序列化 Confirmation 失败：{e}, raw={payload}",
                    msg.unique_id
                ),
            })
        }
        Some(t) if t == CALLERROR => {
            let code = arr.get(2).and_then(|v| v.as_str()).unwrap_or("Unknown");
            let desc = arr.get(3).and_then(|v| v.as_str()).unwrap_or("");
            Err(AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!("请求ID {}, 桩返回 CALLERROR [{code}]: {desc}", msg.unique_id),
            })
        }
        other => Err(AppError::OCPP_1_6_ERROR {
            action: action_label.to_string(),
            detail: format!("请求ID {}, 未知 MQ 响应类型：{other:?}", msg.unique_id),
        }),
    }
}

/// HTTP 出站通用流程：组装 CALL → POST 到 envelope 中的 HTTP URL → 解析响应 → 反序列化为 `T`。
///
/// 协议细节同 [`dispatch_mq_call`]：出站 `[2, unique_id, action, payload]`，
/// 入站 `[3, unique_id, payload]` 或 `[4, unique_id, code, desc, detail]`。
///
/// `action_label` 仅用于错误信息中的可读字段，不参与协议。
#[cfg(all(feature = "ocpp_1_6", feature = "message_by_http"))]
pub async fn dispatch_http_call<T>(
    state: &AppState,
    msg: &CloudMessage,
    action_label: &str,
) -> Result<T, AppError>
where
    T: for<'de> Deserialize<'de>,
{
    let sender = state.http_sender().map_err(|e| AppError::OCPP_1_6_ERROR {
        action: action_label.to_string(),
        detail: format!("请求ID {}, 获取 HTTP sender 失败：{e}", msg.unique_id),
    })?;

    let ocpp_call = serde_json::json!([CALL, &msg.unique_id, msg.action, msg.payload]);

    let resp_value = sender
        .post_ocpp(&msg.csms_request_cp_message_http_url, &ocpp_call)
        .await
        .map_err(|e| AppError::OCPP_1_6_ERROR {
            action: action_label.to_string(),
            detail: format!("请求ID {}, HTTP 请求失败：{e}", msg.unique_id),
        })?;

    let arr = resp_value.as_array().ok_or_else(|| AppError::OCPP_1_6_ERROR {
        action: action_label.to_string(),
        detail: format!(
            "请求ID {}, OCPP 响应不是有效数组：{resp_value}",
            msg.unique_id
        ),
    })?;

    match arr.first().and_then(|v| v.as_i64()) {
        Some(t) if t == CALLRESULT => {
            let payload = arr.get(2).ok_or_else(|| AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!(
                    "请求ID {}, CALLRESULT 缺少 payload 字段：{resp_value}",
                    msg.unique_id
                ),
            })?;
            serde_json::from_value(payload.clone()).map_err(|e| AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!(
                    "请求ID {}, 反序列化 Confirmation 失败：{e}, raw={payload}",
                    msg.unique_id
                ),
            })
        }
        Some(t) if t == CALLERROR => {
            let code = arr.get(2).and_then(|v| v.as_str()).unwrap_or("Unknown");
            let desc = arr.get(3).and_then(|v| v.as_str()).unwrap_or("");
            Err(AppError::OCPP_1_6_ERROR {
                action: action_label.to_string(),
                detail: format!("请求ID {}, 桩返回 CALLERROR [{code}]: {desc}", msg.unique_id),
            })
        }
        other => Err(AppError::OCPP_1_6_ERROR {
            action: action_label.to_string(),
            detail: format!("请求ID {}, 未知 OCPP 消息类型：{other:?}", msg.unique_id),
        }),
    }
}

pub struct UnknownRequest;
impl Handler<String> for UnknownRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(_state: &AppState, msg: &CloudMessage) -> Result<String, AppError> {
        let action = msg.action.as_str();
        Err(AppError::OCPP_1_6_ERROR {
            action: action.to_string(),
            detail: "not implemented".to_string(),
        })
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(_state: &AppState, msg: &CloudMessage) -> Result<String, AppError> {
        let action = msg.action.as_str();
        Err(AppError::OCPP_1_6_ERROR {
            action: action.to_string(),
            detail: "not implemented".to_string(),
        })
    }
}
