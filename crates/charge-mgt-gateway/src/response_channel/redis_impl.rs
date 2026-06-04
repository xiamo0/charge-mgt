use redis::aio::MultiplexedConnection;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::config::RedisConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;

use ocpp_1_6::envelope::{CallError, CallResult};

pub struct RedisResponseChannel {
    conn: MultiplexedConnection,
    timeout_secs: u64,
}

impl RedisResponseChannel {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let client = redis::Client::open(config.url.as_str())
            .map_err(|e| GatewayError::Redis(format!("Failed to create client: {}", e)))?;
        let conn = client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|e| GatewayError::Redis(format!("Failed to connect: {}", e)))?;

        info!("Redis connected to {}", config.url);
        Ok(Self {
            conn,
            timeout_secs: config.response_timeout_secs,
        })
    }
}

impl ResponseChannel for RedisResponseChannel {
    fn dispatch_pending_request(
        &self,
        unique_id: String,
        charge_point_id: String,
        action: String,
        response_tx: mpsc::UnboundedSender<String>,
    ) {
        let conn = self.conn.clone();
        let timeout_secs = self.timeout_secs;

        tokio::spawn(async move {
            let key = format!("resp:{}", unique_id);
            info!(
                "[REDIS] BLPOP waiting: key={}, action={}, cp={}, timeout={}s",
                key, action, charge_point_id, timeout_secs
            );

            let result: std::result::Result<Option<(String, String)>, redis::RedisError> =
                redis::cmd("BLPOP")
                    .arg(&key)
                    .arg(timeout_secs as f64)
                    .query_async(&mut conn.clone())
                    .await;

            match result {
                Ok(Some((_, payload))) => {
                    info!(
                        "[REDIS] Response received: key={}, cp={}",
                        key, charge_point_id
                    );
                    let payload_value: serde_json::Value =
                        serde_json::from_str(&payload).unwrap_or(serde_json::json!({}));
                    let call_result = CallResult::new(&unique_id, payload_value);
                    let json = serde_json::to_string(&call_result).unwrap_or_default();
                    if response_tx.send(json).is_err() {
                        warn!(
                            "[REDIS] Failed to send response to CP {} (channel closed)",
                            charge_point_id
                        );
                    }
                    let _: () = redis::cmd("DEL")
                        .arg(&key)
                        .query_async(&mut conn.clone())
                        .await
                        .unwrap_or(());
                }
                Ok(None) => {
                    warn!(
                        "[REDIS] Timeout: key={}, cp={}, action={}",
                        key, charge_point_id, action
                    );
                    let call_error = CallError::new(
                        &unique_id,
                        "InternalError",
                        "Cloud platform response timeout",
                    );
                    let json = serde_json::to_string(&call_error).unwrap_or_default();
                    response_tx.send(json).ok();
                }
                Err(e) => {
                    error!(
                        "[REDIS] Error: key={}, cp={}, error={}",
                        key, charge_point_id, e
                    );
                    let call_error = CallError::new(
                        &unique_id,
                        "InternalError",
                        &format!("Redis error: {}", e),
                    );
                    let json = serde_json::to_string(&call_error).unwrap_or_default();
                    response_tx.send(json).ok();
                }
            }
        });
    }
}