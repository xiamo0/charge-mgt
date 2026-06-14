use sea_orm::sea_query::Value;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use tracing::{info, warn};

use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::ocpp::handlers::{boot_notification, heartbeat};
use crate::state::AppState;

pub struct MessageDispatcher {
    state: AppState,
}

impl MessageDispatcher {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn dispatch(&self, bytes: &[u8]) -> Result<(), DispatchError> {
        let msg: CloudMessage = serde_json::from_slice(bytes)
            .map_err(|e| DispatchError::Malformed(e.to_string()))?;

        if !msg.is_call() {
            info!(
                message_type = %msg.message_type,
                unique_id = %msg.unique_id,
                "skipping non-Call message"
            );
            return Ok(());
        }

        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "INSERT INTO charge_mgt_sent_messages_ocpp_1_6 \
             (unique_id, gateway_id, charge_point_id, direction, action, message_type) \
             VALUES ($1, $2, $3, 'inbound', $4, $5) \
             ON CONFLICT (unique_id) DO NOTHING \
             RETURNING true",
            vec![
                Value::String(Some(Box::new(msg.unique_id.clone()))),
                Value::String(Some(Box::new(msg.gateway_id.clone()))),
                Value::String(Some(Box::new(msg.charge_point_id.clone()))),
                Value::String(Some(Box::new(msg.action.clone()))),
                Value::String(Some(Box::new(msg.message_type.clone()))),
            ],
        );

        let inserted = self
            .state
            .db
            .query_one(stmt)
            .await
            .map_err(|e| DispatchError::Database(e.to_string()))?
            .is_some();

        if !inserted {
            info!(
                unique_id = %msg.unique_id,
                "duplicate message, skipping (idempotency)"
            );
            return Ok(());
        }

        info!(
            action = %msg.action,
            charge_point_id = %msg.charge_point_id,
            unique_id = %msg.unique_id,
            "dispatching OCPP Call"
        );

        let handler_result = Self::route_handler(&self.state, &msg).await;

        let response = match handler_result {
            Ok(payload) => msg.new_call_result(payload),
            Err(e) => {
                let (code, desc) = e.to_ocpp_error();
                warn!(
                    action = %msg.action,
                    unique_id = %msg.unique_id,
                    error_code = %code,
                    "handler returned CallError"
                );
                msg.new_call_error(code, &desc)
            }
        };

        let resp_bytes = serde_json::to_vec(&response)
            .map_err(|e| DispatchError::Serialize(e.to_string()))?;

        let topic = self.resp_topic(&msg.gateway_id);
        self.state
            .producer
            .send_resp(&topic, &msg.unique_id, &resp_bytes)
            .await
            .map_err(DispatchError::Kafka)?;

        Ok(())
    }

    async fn route_handler(state: &AppState, msg: &CloudMessage) -> Result<serde_json::Value, HandlerError> {
        match msg.action.as_str() {
            "BootNotification" => boot_notification::handle(state, msg).await,
            "Heartbeat" => heartbeat::handle(state, msg).await,
            other => {
                warn!(action = %other, "unsupported OCPP action");
                Err(HandlerError::NotSupported(format!("action '{other}' not implemented")))
            }
        }
    }

    fn resp_topic(&self, gateway_id: &str) -> String {
        format!("{}.resp.{}", self.state.config.kafka.topic_prefix, gateway_id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("malformed message: {0}")]
    Malformed(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("kafka send error: {0}")]
    Kafka(String),
}
