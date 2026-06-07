//! Redis 响应通道实现
//!
//! 云端将响应写入 `resp:{uniqueId}` 键后，网关通过 BLPOP 阻塞获取并回传充电桩。

use redis::aio::MultiplexedConnection;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::config::RedisConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;

use ocpp_1_6::envelope::{CallError, CallResult};

/// 基于 Redis BLPOP 的响应通道
pub struct RedisResponseChannel {
    /// Redis 多路复用连接
    conn: MultiplexedConnection,
    /// BLPOP 等待云端响应的超时时间（秒）
    timeout_secs: u64,
}

impl RedisResponseChannel {
    /// 连接 Redis 并创建响应通道
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let client = redis::Client::open(config.url.as_str())
            .map_err(|e| GatewayError::Redis(format!("Failed to create client: {}", e)))?;
        let conn = client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|e| GatewayError::Redis(format!("Failed to connect: {}", e)))?;

        info!("Redis 已连接: {}", config.url);
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
                "[REDIS] BLPOP 等待中: 键={}, 动作={}, 充电桩={}, 超时={}秒",
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
                        "[REDIS] 收到响应: 键={}, 充电桩={}",
                        key, charge_point_id
                    );
                    let payload_value: serde_json::Value =
                        serde_json::from_str(&payload).unwrap_or(serde_json::json!({}));
                    let call_result = CallResult::new(&unique_id, payload_value);
                    let json = serde_json::to_string(&call_result).unwrap_or_default();
                    if response_tx.send(json).is_err() {
                        warn!(
                            "[REDIS] 响应发送失败，充电桩 {}（通道已关闭）",
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
                        "[REDIS] 等待超时: 键={}, 充电桩={}, 动作={}",
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
                        "[REDIS] 错误: 键={}, 充电桩={}, 错误={}",
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