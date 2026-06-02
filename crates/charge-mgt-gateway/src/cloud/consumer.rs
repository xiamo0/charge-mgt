use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use std::sync::Arc;
use futures_util::StreamExt;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::cloud::message::CloudMessage;
use crate::error::{GatewayError, Result};

#[derive(Clone)]
pub struct ConnectionMeta {
    pub charge_point_id: String,
    pub vendor: String,
    pub protocol_version: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
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
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    connection_manager: Arc<ConnectionManager>,
    topic_prefix: String,
}

impl KafkaConsumer {
    pub fn new(
        brokers: &str,
        group_id: &str,
        topic_prefix: &str,
        connection_manager: Arc<ConnectionManager>,
    ) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "5000")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("Failed to create consumer: {}", e)))?;

        let topics = vec![
            format!("{}.Alphas", topic_prefix),
            format!("{}.unknown", topic_prefix),
        ];
        let topic_refs: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
        consumer
            .subscribe(&topic_refs)
            .map_err(|e| GatewayError::Kafka(format!("Failed to subscribe: {}", e)))?;

        info!("Kafka consumer subscribed to topics: {:?}", topics);

        Ok(Self {
            consumer,
            connection_manager,
            topic_prefix: topic_prefix.to_string(),
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
                                self.handle_cloud_message(cloud_msg).await;
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

    async fn handle_cloud_message(&self, msg: CloudMessage) {
        let charge_point_id = &msg.charge_point_id;

        info!(
            "Received cloud message from Kafka: action={}, charge_point_id={}",
            msg.action, charge_point_id
        );

        let is_connected = {
            let connections = self.connection_manager.connections.read().await;
            connections.contains_key(charge_point_id)
        };

        if is_connected {
            let ocpp_call = build_ocpp_call(&msg);
            info!(
                "[DOWNSTREAM] Would send to charge point {}: {}",
                charge_point_id, ocpp_call
            );
        } else {
            warn!(
                "[DOWNSTREAM] Charge point {} not connected, message dropped",
                charge_point_id
            );
        }
    }
}

fn build_ocpp_call(msg: &CloudMessage) -> String {
    let call = ocpp_1_6::envelope::Call::new(
        &msg.action,
        &msg.unique_id,
        msg.payload.clone(),
    );
    serde_json::to_string(&call).unwrap_or_default()
}
