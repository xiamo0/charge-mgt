use chrono::Utc;
use serde::Serialize;

use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(rename = "currentTime")]
    pub current_time: String,
}

pub async fn handle(state: &AppState, msg: &CloudMessage) -> Result<serde_json::Value, HandlerError> {
    sqlx::query(
        r#"
        UPDATE charge_mgt_charge_points_ocpp_1_6
        SET last_heartbeat_at = now(),
            ocpp_status = 'Online',
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(&msg.charge_point_id)
    .execute(&state.db)
    .await?;

    let response = Response {
        current_time: Utc::now().to_rfc3339(),
    };

    Ok(serde_json::to_value(&response)?)
}
