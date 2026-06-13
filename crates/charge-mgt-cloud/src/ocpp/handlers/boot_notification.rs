use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "chargePointVendor")]
    pub vendor: String,
    #[serde(rename = "chargePointModel")]
    pub model: String,
    #[serde(rename = "chargeBoxSerialNumber")]
    pub charge_box_serial_number: Option<String>,
    #[serde(rename = "chargePointSerialNumber")]
    pub charge_point_serial_number: Option<String>,
    #[serde(rename = "firmwareVersion")]
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    #[serde(rename = "meterType")]
    pub meter_type: Option<String>,
    #[serde(rename = "meterSerialNumber")]
    pub meter_serial_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: String,
    #[serde(rename = "currentTime")]
    pub current_time: String,
    pub interval: i32,
}

pub async fn handle(state: &AppState, msg: &CloudMessage) -> Result<serde_json::Value, HandlerError> {
    let req: Request = serde_json::from_value(msg.payload.clone())?;

    if req.vendor.len() > 20 {
        return Err(HandlerError::FormationViolation(
            "chargePointVendor must be <= 20 characters".into(),
        ));
    }
    if req.model.len() > 20 {
        return Err(HandlerError::FormationViolation(
            "chargePointModel must be <= 20 characters".into(),
        ));
    }

    let cp_id = req
        .charge_box_serial_number
        .clone()
        .or_else(|| req.charge_point_serial_number.clone())
        .ok_or_else(|| {
            HandlerError::FormationViolation(
                "chargePointSerialNumber/chargeBoxSerialNumber required".into(),
            )
        })?;

    let insert_result = sqlx::query(
        r#"
        INSERT INTO charge_mgt_charge_points_ocpp_1_6 (
            id, gateway_id, gateway_ip, vendor, model,
            serial_number, charge_box_serial,
            firmware_version, iccid, imsi, meter_type, meter_serial_number,
            protocol_version, ocpp_status, last_boot_at, last_heartbeat_at,
            registered_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7,
            $8, $9, $10, $11, $12,
            'OCPP-1.6', 'Online', now(), now(),
            now(), now()
        )
        ON CONFLICT (id) DO UPDATE SET
            firmware_version = EXCLUDED.firmware_version,
            gateway_id = EXCLUDED.gateway_id,
            gateway_ip = EXCLUDED.gateway_ip,
            ocpp_status = 'Online',
            last_boot_at = now(),
            last_heartbeat_at = now(),
            updated_at = now()
        "#,
    )
    .bind(&cp_id)
    .bind(&msg.gateway_id)
    .bind(&msg.gateway_ip)
    .bind(&req.vendor)
    .bind(&req.model)
    .bind(&req.charge_point_serial_number)
    .bind(&req.charge_box_serial_number)
    .bind(&req.firmware_version)
    .bind(&req.iccid)
    .bind(&req.imsi)
    .bind(&req.meter_type)
    .bind(&req.meter_serial_number)
    .execute(&state.db)
    .await?;

    let _ = insert_result;

    sqlx::query(
        r#"
        INSERT INTO charge_mgt_connectors_ocpp_1_6 (charge_point_id, connector_id, status)
        VALUES ($1, 0, 'Available')
        ON CONFLICT (charge_point_id, connector_id) DO NOTHING
        "#,
    )
    .bind(&cp_id)
    .execute(&state.db)
    .await?;

    let response = Response {
        status: "Accepted".into(),
        current_time: Utc::now().to_rfc3339(),
        interval: 30,
    };

    Ok(serde_json::to_value(&response)?)
}
