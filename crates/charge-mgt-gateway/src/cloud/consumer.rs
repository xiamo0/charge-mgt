use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use std::sync::Arc;
use futures_util::StreamExt;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info, warn};

use crate::cloud::message::CloudMessage;
use crate::config::KafkaConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::{PendingRequestTracker, MessageDirection};

use ocpp_1_6::envelope::{Call, CallError, CallResult};

#[derive(Clone)]
pub struct ConnectionMeta {
    pub charge_point_id: String,
    pub vendor: String,
    pub protocol_version: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub response_tx: mpsc::UnboundedSender<String>,
}

pub struct ConnectionManager {
    connections: Arc<RwLock<std::collections::HashMap<String, ConnectionMeta>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn add_connection(&self, charge_point_id: String, meta: ConnectionMeta) {
        let mut connections = self.connections.write().await;
        connections.insert(charge_point_id, meta);
    }

    pub async fn update_connection(&self, charge_point_id: &str, meta: ConnectionMeta) {
        let mut connections = self.connections.write().await;
        connections.insert(charge_point_id.to_string(), meta);
    }

    pub async fn remove_connection(&self, charge_point_id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(charge_point_id);
    }

    pub async fn list_charge_points(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    pub async fn get_connection_info(&self, charge_point_id: &str) -> Option<ConnectionMeta> {
        let connections = self.connections.read().await;
        connections.get(charge_point_id).cloned()
    }

    pub async fn send_to_charge_point(&self, charge_point_id: &str, message: String) -> bool {
        match self.get_connection_info(charge_point_id).await {
            Some(meta) => {
                if meta.response_tx.send(message).is_ok() {
                    info!("Message sent to charge point {}", charge_point_id);
                    true
                } else {
                    warn!(
                        "Failed to send to charge point {} (channel closed)",
                        charge_point_id
                    );
                    false
                }
            }
            None => {
                warn!("Charge point {} not connected, message dropped", charge_point_id);
                false
            }
        }
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    connection_manager: Arc<ConnectionManager>,
    pending_tracker: Option<Arc<PendingRequestTracker>>,
    topic_prefix: String,
    resp_topic_suffix: Option<String>,
    gateway_id: String,
}

impl KafkaConsumer {
    pub fn new_redis_mode(
        config: &KafkaConfig,
        gateway_id: &str,
        connection_manager: Arc<ConnectionManager>,
    ) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", format!("gateway-{}-cmd-consumer", gateway_id))
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "5000")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("Failed to create consumer: {}", e)))?;

        let cmd_topic = CloudMessage::cmd_topic(&config.topic_prefix, &config.cmd_topic_suffix, gateway_id);
        consumer
            .subscribe(&[&cmd_topic])
            .map_err(|e| GatewayError::Kafka(format!("Failed to subscribe: {}", e)))?;

        info!("Kafka consumer (redis mode) subscribed to: {}", cmd_topic);

        Ok(Self {
            consumer,
            connection_manager,
            pending_tracker: None,
            topic_prefix: config.topic_prefix.clone(),
            resp_topic_suffix: None,
            gateway_id: gateway_id.to_string(),
        })
    }

    pub fn new_kafka_mode(
        config: &KafkaConfig,
        gateway_id: &str,
        connection_manager: Arc<ConnectionManager>,
        pending_tracker: Arc<PendingRequestTracker>,
    ) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", format!("gateway-{}-resp-consumer", gateway_id))
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "5000")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("Failed to create consumer: {}", e)))?;

        let resp_topic = CloudMessage::resp_topic(&config.topic_prefix, &config.resp_topic_suffix, gateway_id);
        consumer
            .subscribe(&[&resp_topic])
            .map_err(|e| GatewayError::Kafka(format!("Failed to subscribe: {}", e)))?;

        info!("Kafka consumer (kafka mode) subscribed to: {}", resp_topic);

        Ok(Self {
            consumer,
            connection_manager,
            pending_tracker: Some(pending_tracker),
            topic_prefix: config.topic_prefix.clone(),
            resp_topic_suffix: Some(config.resp_topic_suffix.clone()),
            gateway_id: gateway_id.to_string(),
        })
    }

    pub async fn run(&self) {
        info!("Kafka consumer started");
        let stream = self.consumer.stream();

        futures_util::pin_mut!(stream);

        while let Some(msg_result) = stream.next().await {
            match msg_result {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        match serde_json::from_slice::<CloudMessage>(payload) {
                            Ok(cloud_msg) => {
                                self.handle_downstream_message(cloud_msg).await;
                            }
                            Err(e) => {
                                warn!("Failed to parse cloud message: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Kafka error: {}", e);
                }
            }
        }
    }

    async fn handle_downstream_message(&self, msg: CloudMessage) {
        let charge_point_id = &msg.charge_point_id;

        info!(
            "Received downstream message: message_type={}, action={}, charge_point_id={}",
            msg.message_type, msg.action, charge_point_id
        );

        match msg.message_type.as_str() {
            "CallResult" => self.handle_call_result(&msg, charge_point_id).await,
            "CallError" => self.handle_call_error(&msg, charge_point_id).await,
            "Call" => self.handle_call(&msg, charge_point_id).await,
            _ => warn!("Unknown message_type: {}", msg.message_type),
        }
    }

    async fn handle_call_result(&self, msg: &CloudMessage, charge_point_id: &str) {
        let tracker = match &self.pending_tracker {
            Some(t) => t,
            None => {
                warn!("CallResult received but no pending tracker (redis mode should not receive CallResult via Kafka)");
                self.connection_manager
                    .send_to_charge_point(
                        charge_point_id,
                        build_ocpp_call_result(&msg.unique_id, msg.payload.clone()),
                    )
                    .await;
                return;
            }
        };

        let pending = tracker.remove(&msg.unique_id).await;
        match pending {
            Some(request) => {
                info!(
                    "Cloud CallResult matched pending request: uniqueId={}, action={}",
                    request.unique_id, request.action
                );
                let call_result_json = build_ocpp_call_result(&msg.unique_id, msg.payload.clone());
                request.response_tx.send(call_result_json).ok();
            }
            None => {
                warn!(
                    "No pending request for CallResult uniqueId={}, forwarding directly to CP",
                    msg.unique_id
                );
                self.connection_manager
                    .send_to_charge_point(
                        charge_point_id,
                        build_ocpp_call_result(&msg.unique_id, msg.payload.clone()),
                    )
                    .await;
            }
        }
    }

    async fn handle_call_error(&self, msg: &CloudMessage, charge_point_id: &str) {
        let tracker = match &self.pending_tracker {
            Some(t) => t,
            None => {
                let error_json = build_ocpp_call_error(
                    &msg.unique_id,
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                self.connection_manager.send_to_charge_point(charge_point_id, error_json).await;
                return;
            }
        };

        let pending = tracker.remove(&msg.unique_id).await;
        match pending {
            Some(request) => {
                info!(
                    "Cloud CallError matched pending request: uniqueId={}, action={}",
                    request.unique_id, request.action
                );
                let error_json = build_ocpp_call_error(
                    &msg.unique_id,
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                request.response_tx.send(error_json).ok();
            }
            None => {
                let error_json = build_ocpp_call_error(
                    &msg.unique_id,
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                self.connection_manager.send_to_charge_point(charge_point_id, error_json).await;
            }
        }
    }

    async fn handle_call(&self, msg: &CloudMessage, charge_point_id: &str) {
        let call_json = build_ocpp_call(&msg.action, &msg.unique_id, msg.payload.clone());

        if self.connection_manager.send_to_charge_point(charge_point_id, call_json).await {
            if let Some(tracker) = &self.pending_tracker {
                let meta = self.connection_manager.get_connection_info(charge_point_id).await;
                if let Some(meta) = meta {
                    tracker
                        .register(crate::response_channel::PendingRequest {
                            unique_id: msg.unique_id.clone(),
                            charge_point_id: charge_point_id.to_string(),
                            action: msg.action.clone(),
                            direction: MessageDirection::Downstream,
                            created_at: std::time::Instant::now(),
                            response_tx: meta.response_tx,
                        })
                        .await;
                }
            }
        }
    }
}

fn build_ocpp_call(action: &str, unique_id: &str, payload: serde_json::Value) -> String {
    let call = Call::new(action, unique_id, payload);
    serde_json::to_string(&call).unwrap_or_default()
}

fn build_ocpp_call_result(unique_id: &str, payload: serde_json::Value) -> String {
    let call_result = CallResult::new(unique_id, payload);
    serde_json::to_string(&call_result).unwrap_or_default()
}

fn build_ocpp_call_error(unique_id: &str, error_code: &str, error_description: &str) -> String {
    let call_error = CallError::new(unique_id, error_code, error_description);
    serde_json::to_string(&call_error).unwrap_or_default()
}