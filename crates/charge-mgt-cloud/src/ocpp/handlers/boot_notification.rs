use chrono::Utc;
use sea_orm::sea_query::{OnConflict, Value};
use sea_orm::{ActiveValue::Set, ConnectionTrait, DatabaseBackend, EntityTrait, Statement};
use serde::{Deserialize, Serialize};

use crate::entity::{
    charge_points::{ActiveModel as CpActiveModel, Column as CpColumn},
    ChargePoints,
};
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

    let now = Utc::now().into();

    let cp = CpActiveModel {
        id: Set(cp_id.clone()),
        gateway_id: Set(msg.gateway_id.clone()),
        gateway_ip: Set(Some(msg.gateway_ip.clone())),
        vendor: Set(req.vendor.clone()),
        model: Set(req.model.clone()),
        serial_number: Set(req.charge_point_serial_number.clone()),
        charge_box_serial: Set(req.charge_box_serial_number.clone()),
        firmware_version: Set(req.firmware_version.clone()),
        iccid: Set(req.iccid.clone()),
        imsi: Set(req.imsi.clone()),
        meter_type: Set(req.meter_type.clone()),
        meter_serial_number: Set(req.meter_serial_number.clone()),
        protocol_version: Set("OCPP-1.6".to_string()),
        ocpp_status: Set("Online".to_string()),
        heartbeat_interval_secs: Set(30),
        last_heartbeat_at: Set(Some(now)),
        last_boot_at: Set(Some(now)),
        registered_at: Set(now),
        updated_at: Set(now),
        is_deleted: Set(false),
    };

    let mut on_cp_conflict = OnConflict::column(CpColumn::Id);
    on_cp_conflict.update_columns([
        CpColumn::FirmwareVersion,
        CpColumn::GatewayId,
        CpColumn::GatewayIp,
        CpColumn::OcppStatus,
        CpColumn::LastBootAt,
        CpColumn::LastHeartbeatAt,
        CpColumn::UpdatedAt,
    ]);

    ChargePoints::insert(cp)
        .on_conflict(on_cp_conflict)
        .exec(&state.db)
        .await?;

    let conn_stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "INSERT INTO charge_mgt_connectors_ocpp_1_6 (charge_point_id, connector_id, status) \
         VALUES ($1, $2, $3::charge_mgt_connector_status) \
         ON CONFLICT (charge_point_id, connector_id) DO NOTHING",
        vec![
            Value::String(Some(Box::new(cp_id))),
            Value::Int(Some(0)),
            Value::String(Some(Box::new("Available".to_string()))),
        ],
    );
    state.db.execute(conn_stmt).await?;

    let response = Response {
        status: "Accepted".into(),
        current_time: Utc::now().to_rfc3339(),
        interval: 30,
    };

    Ok(serde_json::to_value(&response)?)
}
