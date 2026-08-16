//! 处理充电桩上报的 DiagnosticsStatusNotification 消息。
//!
//! 该消息为 OCPP 1.6 的单向通知：充电桩在诊断上传/下载流程中上报当前状态（Idle /
//! Downloading / Downloaded / Installing / Installed / UploadFailed）。
//! CSMS 收到后只需记录，无需回业务响应（确认体为空 unit struct）。

use serde_json::json;
use tracing::{info, warn};

use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_DIAGNOSTICS_STATUS_NOTIFICATION_CONFIRMATION;
use ocpp_1_6::calls::DiagnosticsStatusNotificationRequest;
use ocpp_1_6::confs::DiagnosticsStatusNotificationConfirmation;

impl Handler<DiagnosticsStatusNotificationConfirmation> for DiagnosticsStatusNotificationRequest {
    async fn handel_detail(
        _state: &AppState,
        msg: &CloudMessage,
    ) -> Result<DiagnosticsStatusNotificationConfirmation, AppError> {
        let req: DiagnosticsStatusNotificationRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        // CSMS 端当前 phase 0 schema 没有 diagnostics_status 表，先记录结构化 trace。
        // 真实持久化等 phase 1 引入 mgt_diagnostics_status_ocpp_1_6 后再补。
        let charge_point_id = msg.charge_point_id.as_deref().unwrap_or("");
        if charge_point_id.is_empty() {
            warn!(
                unique_id = %msg.unique_id.as_deref().unwrap_or(""),
                "DiagnosticsStatusNotification 缺少 charge_point_id"
            );
        }

        // payload 字段在云端收到时是 enum，序列化后用 PascalCase 字符串
        let status_str = serde_json::to_value(&req.status)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        info!(
            unique_id = %msg.unique_id.as_deref().unwrap_or(""),
            charge_point_id = %charge_point_id,
            diagnostics_status = %status_str,
            request_id = ?req.request_id,
            payload = %serde_json::to_string(&json!({
                "status": status_str,
                "request_id": req.request_id,
            })).unwrap_or_default(),
            "DiagnosticsStatusNotification received"
        );

        Ok(DiagnosticsStatusNotificationConfirmation)
    }
}