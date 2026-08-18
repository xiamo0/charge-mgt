//! 处理充电桩上报的 DataTransfer 消息。
//!
//! DataTransfer 是 OCPP 1.6 的厂商扩展点：充电桩可通过它传递厂商自定义的任意 JSON
//! 数据给 CSMS。CSMS 端通常会注册 vendor-specific 处理器来决定如何处理。
//!
//! 当前 cloud 端没有 vendor 处理器注册表，因此默认返回 `UnknownVendorId`：
//! 既不假装处理（避免数据丢失或误处理），也不主动拒绝（让上游 retry 链路有机会
//! 走通）。真正生产环境应支持按 `vendor_id` 注册 handler。

use tracing::{info, warn};

use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_DATA_TRANSFER;
use ocpp_1_6::calls::DataTransferRequest;
use ocpp_1_6::confs::DataTransferConfirmation;

impl Handler<DataTransferConfirmation> for DataTransferRequest {
    async fn handel_detail(
        _state: &AppState,
        msg: &CloudMessage,
    ) -> Result<DataTransferConfirmation, AppError> {
        let req: DataTransferRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        // 当前 phase 0 没有 vendor 处理器注册表：记录结构化 trace 后返回 UnknownVendorId。
        // 真正生产环境应通过 vendor_id 注册回调处理器。
        if req.vendor_id.is_empty() {
            warn!(
                unique_id = %msg.unique_id.as_deref().unwrap_or(""),
                charge_point_id = %msg.charge_point_id.as_deref().unwrap_or(""),
                "DataTransfer 缺少 vendor_id"
            );
            return Ok(DataTransferConfirmation::rejected());
        }

        info!(
            unique_id = %msg.unique_id.as_deref().unwrap_or(""),
            charge_point_id = %msg.charge_point_id.as_deref().unwrap_or(""),
            vendor_id = %req.vendor_id,
            message_id = ?req.message_id,
            has_data = req.data.is_some(),
            action = %ACTION_DATA_TRANSFER,
            "DataTransfer received (no vendor handler registered; returning UnknownVendorId)"
        );

        Ok(DataTransferConfirmation::unknown_vendor_id())
    }
}