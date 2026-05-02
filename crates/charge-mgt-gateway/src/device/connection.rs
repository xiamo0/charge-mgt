use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::cloud::{CloudMessage, CloudMessageInput, MockKafkaProducer};
use crate::config::DeviceConfig;
use crate::error::{GatewayError, Result};
use crate::state::AppState;

type Call = ocpp_1_6::envelope::Call;
type CallResult = ocpp_1_6::envelope::CallResult;
type CallError = ocpp_1_6::envelope::CallError;

#[allow(dead_code)]
pub struct Connection {
    pub id: String,
    pub addr: SocketAddr,
    config: DeviceConfig,
    charge_point_vendor: Option<String>,
    charge_point_model: Option<String>,
    charge_point_id: Option<String>,
    state: Arc<AppState>,
    kafka_producer: Arc<MockKafkaProducer>,
    gateway_id: String,
    gateway_host: String,
}

impl Connection {
    pub fn new(
        addr: SocketAddr,
        state: Arc<AppState>,
        config: DeviceConfig,
        kafka_producer: Arc<MockKafkaProducer>,
        gateway_id: String,
        gateway_host: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            addr,
            charge_point_vendor: None,
            charge_point_model: None,
            charge_point_id: None,
            state,
            config,
            kafka_producer,
            gateway_id,
            gateway_host,
        }
    }

    pub async fn handle_message(&mut self, text: &str) -> Result<Option<String>> {
        info!("Raw message (len={}): {}", text.len(), text);
        let call: Call = match serde_json::from_str(text) {
            Ok(c) => c,
            Err(e) => {
                warn!(
                    "Failed to parse message as OCPP Call: text_len={}, error={}",
                    text.len(),
                    e
                );
                return Err(GatewayError::Codec(format!("Invalid OCPP message: {}", e)));
            }
        };

        info!(
            "Received OCPP Call: action={}, uniqueId={}",
            call.action, call.unique_id
        );

        let cloud_msg = self.build_cloud_message(&call);

        if let Err(e) = self.kafka_producer.send(&cloud_msg).await {
            error!("Failed to send message to Kafka: {}", e);
        }

        let response = match call.action.as_str() {
            "BootNotification" => {
                self.handle_boot_notification(call.unique_id, call.payload)
                    .await?
            }
            "Heartbeat" => self.handle_heartbeat(call.unique_id).await?,
            "Authorize" => self.handle_authorize(call.unique_id, call.payload).await?,
            "StartTransaction" => {
                self.handle_start_transaction(call.unique_id, call.payload)
                    .await?
            }
            "StopTransaction" => {
                self.handle_stop_transaction(call.unique_id, call.payload)
                    .await?
            }
            "MeterValues" => {
                self.handle_meter_values(call.unique_id, call.payload)
                    .await?
            }
            "StatusNotification" => {
                self.handle_status_notification(call.unique_id, call.payload)
                    .await?
            }
            _ => {
                warn!("Unsupported action: {}", call.action);
                return Ok(Some(self.create_call_error(
                    &call.unique_id,
                    "NotImplemented",
                    &format!("Action {} not implemented", call.action),
                )));
            }
        };

        Ok(response)
    }

    fn build_cloud_message(&self, call: &Call) -> CloudMessage {
        let input = CloudMessageInput {
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_host.clone(),
            vendor: self
                .charge_point_vendor
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            charge_point_id: self
                .charge_point_id
                .clone()
                .unwrap_or_else(|| self.id.clone()),
            protocol: "OCPP-1.6".to_string(),
            message_type: "Call".to_string(),
            action: call.action.clone(),
            unique_id: call.unique_id.clone(),
        };
        CloudMessage::new(input, call.payload.clone())
    }

    async fn handle_boot_notification(
        &mut self,
        unique_id: String,
        payload: Value,
    ) -> Result<Option<String>> {
        info!(
            "handle_boot_notification called, unique_id={}, payload={}",
            unique_id, payload
        );
        let charge_point_vendor = payload
            .get("chargePointVendor")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let charge_point_model = payload
            .get("chargePointModel")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let charge_box_serial = payload
            .get("chargeBoxSerialNumber")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        info!(
            "BootNotification: vendor={}, model={}, chargeBoxSerial={:?}",
            charge_point_vendor, charge_point_model, charge_box_serial
        );

        self.charge_point_vendor = Some(charge_point_vendor.clone());
        self.charge_point_model = Some(charge_point_model.clone());
        self.charge_point_id = charge_box_serial.clone();

        let charge_point_id = self
            .charge_point_id
            .clone()
            .unwrap_or_else(|| self.id.clone());

        self.state
            .register_charge_point(
                charge_point_id.clone(),
                charge_point_vendor.clone(),
                "OCPP-1.6".to_string(),
            )
            .await;

        let response_payload = serde_json::json!({
            "status": "Accepted",
            "currentTime": chrono::Utc::now().to_rfc3339(),
            "interval": 30
        });

        let response = CallResult::new(&unique_id, response_payload);
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_heartbeat(&self, unique_id: String) -> Result<Option<String>> {
        let response_payload = serde_json::json!({
            "currentTime": chrono::Utc::now().to_rfc3339()
        });

        let response = CallResult::new(&unique_id, response_payload);
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_authorize(&self, unique_id: String, payload: Value) -> Result<Option<String>> {
        let id_tag = payload
            .get("idTag")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        info!("Authorize: idTag={}", id_tag);

        let response_payload = serde_json::json!({
            "status": "Accepted",
            "idTagInfo": {
                "status": "Accepted",
                "expiryDate": null
            }
        });

        let response = CallResult::new(&unique_id, response_payload);
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_start_transaction(
        &self,
        unique_id: String,
        payload: Value,
    ) -> Result<Option<String>> {
        let id_tag = payload
            .get("idTag")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let connector_id = payload
            .get("connectorId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        info!(
            "StartTransaction: idTag={}, connectorId={}",
            id_tag, connector_id
        );

        let transaction_id = rand::random::<i32>().abs();

        let response_payload = serde_json::json!({
            "transactionId": transaction_id,
            "idTagInfo": {
                "status": "Accepted"
            }
        });

        let response = CallResult::new(&unique_id, response_payload);
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_stop_transaction(
        &self,
        unique_id: String,
        payload: Value,
    ) -> Result<Option<String>> {
        let transaction_id = payload
            .get("transactionId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        info!("StopTransaction: transactionId={}", transaction_id);

        let response_payload = serde_json::json!({
            "idTagInfo": {
                "status": "Accepted"
            }
        });

        let response = CallResult::new(&unique_id, response_payload);
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_meter_values(
        &self,
        unique_id: String,
        payload: Value,
    ) -> Result<Option<String>> {
        let transaction_id = payload
            .get("transactionId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let connector_id = payload
            .get("connectorId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        info!(
            "MeterValues: transactionId={}, connectorId={}",
            transaction_id, connector_id
        );

        let response = CallResult::new(&unique_id, serde_json::json!({}));
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    async fn handle_status_notification(
        &self,
        unique_id: String,
        payload: Value,
    ) -> Result<Option<String>> {
        let connector_id = payload
            .get("connectorId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let status = payload
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        info!(
            "StatusNotification: connectorId={}, status={}",
            connector_id, status
        );

        let response = CallResult::new(&unique_id, serde_json::json!({}));
        Ok(Some(
            serde_json::to_string(&response).map_err(|e| GatewayError::Codec(e.to_string()))?,
        ))
    }

    fn create_call_error(&self, unique_id: &str, error_code: &str, error_desc: &str) -> String {
        let error = CallError::new(unique_id, error_code, error_desc);
        serde_json::to_string(&error)
            .unwrap_or_else(|_| r#"[4,"","GenericError","Error",{}]"#.to_string())
    }

    pub async fn on_disconnect(&mut self) {
        if let Some(ref id) = self.charge_point_id {
            info!("Disconnecting charge point: {}", id);
            self.state.remove_charge_point(id).await;
        }
    }
}
