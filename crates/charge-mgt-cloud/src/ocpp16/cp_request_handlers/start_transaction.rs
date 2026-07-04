use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::entity::{ChargePoints, charge_points::Column as CpColumn};
use crate::ocpp16::cp_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::StartTransactionRequest;
use ocpp_1_6::confs::StartTransactionConfirmation;

impl Handler<StartTransactionConfirmation> for StartTransactionRequest {
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<StartTransactionConfirmation, crate::ocpp16::error::HandlerError> {
        todo!()
    }
}
