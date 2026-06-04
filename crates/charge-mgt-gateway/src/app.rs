use crate::cloud::{ConnectionManager, KafkaConsumer, KafkaProducer};
use crate::config::{Config, ResponseChannelMode};
use crate::device::websocket::WebSocketServer;
use crate::error::Result;
use crate::response_channel::{KafkaResponseChannel, PendingRequestTracker, RedisResponseChannel, ResponseChannel};
use std::sync::Arc;
use tracing::info;

pub struct Application {
    config: Config,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    response_channel: Arc<dyn ResponseChannel>,
    pending_tracker: Option<Arc<PendingRequestTracker>>,
}

impl Application {
    pub async fn new(config: Config) -> Result<Self> {
        let kafka_producer = Arc::new(KafkaProducer::new(&config.kafka).await?);
        let connection_manager = Arc::new(ConnectionManager::new());

        let (response_channel, pending_tracker): (Arc<dyn ResponseChannel>, Option<Arc<PendingRequestTracker>>) =
            match config.response_channel {
                ResponseChannelMode::Redis => {
                    let redis_channel = RedisResponseChannel::new(&config.redis).await?;
                    (Arc::new(redis_channel), None)
                }
                ResponseChannelMode::Kafka => {
                    let kafka_channel = KafkaResponseChannel::new(&config.kafka);
                    let tracker = kafka_channel.pending_tracker();
                    (kafka_channel, Some(tracker))
                }
            };

        info!(
            "Starting charge-mgt-gateway application (response_channel={})",
            match config.response_channel {
                ResponseChannelMode::Redis => "redis",
                ResponseChannelMode::Kafka => "kafka",
            }
        );

        Ok(Self {
            config,
            connection_manager,
            kafka_producer,
            response_channel,
            pending_tracker,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting WebSocket server");

        let ws_server = WebSocketServer::new(
            self.config.device.clone(),
            self.connection_manager.clone(),
            self.kafka_producer.clone(),
            self.response_channel.clone(),
            self.config.gateway.id.clone(),
            self.config.gateway.host.clone(),
        );

        let consumer = match (&self.config.response_channel, &self.pending_tracker) {
            (ResponseChannelMode::Redis, None) => {
                KafkaConsumer::new_redis_mode(
                    &self.config.kafka,
                    &self.config.gateway.id,
                    self.connection_manager.clone(),
                )?
            }
            (ResponseChannelMode::Kafka, Some(tracker)) => {
                KafkaConsumer::new_kafka_mode(
                    &self.config.kafka,
                    &self.config.gateway.id,
                    self.connection_manager.clone(),
                    tracker.clone(),
                )?
            }
            _ => {
                return Err(crate::error::GatewayError::Config(
                    "Invalid response_channel/pending_tracker configuration".to_string(),
                ));
            }
        };

        tokio::spawn(async move {
            consumer.run().await;
        });

        ws_server.start().await
    }
}