//! 处理充电桩上报的 FirmwareStatusNotification 消息。
//!
//! 该消息为 OCPP 1.6 的单向通知：充电桩在固件下载/安装流程中上报当前状态
//! （Idle / Downloading / Downloaded / Installing / Installed / DownloadFailed /
//! InstallationFailed）。CSMS 收到后只需记录，无需回业务响应（确认体为空 unit struct）。

use serde_json::json;
use tracing::{info, warn};

use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_FIRMWARE_STATUS_NOTIFICATION_CONFIRMATION;
use ocpp_1_6::calls::FirmwareStatusNotificationRequest;
use ocpp_1_6::confs::FirmwareStatusNotificationConfirmation;

impl Handler<FirmwareStatusNotificationConfirmation> for FirmwareStatusNotificationRequest {
    async fn handel_detail(
        _state: &AppState,
        msg: &CloudMessage,
    ) -> Result<FirmwareStatusNotificationConfirmation, AppError> {
        let req: FirmwareStatusNotificationRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        // CSMS 端当前 phase 0 schema 没有 firmware_status 表，先记录结构化 trace。
        // 真实持久化等 phase 1 引入 mgt_firmware_status_ocpp_1_6 后再补。
        let charge_point_id = msg.charge_point_id.as_deref().unwrap_or("");
        if charge_point_id.is_empty() {
            warn!(
                unique_id = %msg.unique_id.as_deref().unwrap_or(""),
                "FirmwareStatusNotification 缺少 charge_point_id"
            );
        }

        let status_str = serde_json::to_value(&req.status)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        info!(
            unique_id = %msg.unique_id.as_deref().unwrap_or(""),
            charge_point_id = %charge_point_id,
            firmware_status = %status_str,
            request_id = ?req.request_id,
            payload = %serde_json::to_string(&json!({
                "status": status_str,
                "request_id": req.request_id,
            })).unwrap_or_default(),
            "FirmwareStatusNotification received"
        );

        Ok(FirmwareStatusNotificationConfirmation)
    }
}