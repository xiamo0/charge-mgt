use tracing::debug;

use crate::error::AppError;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::MeterValuesRequest;
use ocpp_1_6::confs::MeterValuesConfirmation;

impl Handler<MeterValuesConfirmation> for MeterValuesRequest {
    async fn handel_detail(
        _state: &AppState,
        msg: &CloudMessage,
    ) -> Result<MeterValuesConfirmation, AppError> {
        let req: MeterValuesRequest = serde_json::from_value(msg.payload.clone())?;

        // P1: 当前 phase 0 schema 没有 meter_values 表，先记录 trace。
        // 真实持久化等 phase 1 引入 mgt_meter_values_ocpp_1_6 后再补。
        debug!(
            unique_id = %msg.unique_id,
            charge_point_id = %msg.charge_point_id,
            connector_id = req.connector_id,
            transaction_id = ?req.transaction_id,
            sample_count = req.meter_value.len(),
            "MeterValues received"
        );

        Ok(MeterValuesConfirmation)
    }
}
