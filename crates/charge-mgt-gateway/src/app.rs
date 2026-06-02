use crate::cloud::{ConnectionManager, KafkaConsumer, KafkaProducer};
use crate::config::Config;
use crate::device::websocket::WebSocketServer;
use crate::error::Result;
use std::sync::Arc;
use tracing::info;

pub struct Application {
    config: Config,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
}

impl Application {
    pub async fn new(config: Config) -> Result<Self> {
        let kafka_producer = Arc::new(KafkaProducer::new(&config.kafka).await?);
        let connection_manager = Arc::new(ConnectionManager::new());

        info!("Starting charge-mgt-gateway application");

        Ok(Self {
            config,
            connection_manager,
            kafka_producer,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting WebSocket server");

        let ws_server = WebSocketServer::new(
            self.config.device.clone(),
            self.connection_manager.clone(),
            self.kafka_producer.clone(),
            self.config.gateway.id.clone(),
            self.config.gateway.host.clone(),
        );

        let consumer = KafkaConsumer::new(
            &self.config.kafka.brokers,
            &format!("gateway-{}-consumer", self.config.gateway.id),
            &self.config.kafka.topic_prefix,
            self.connection_manager.clone(),
        )?;

        tokio::spawn(async move {
            consumer.run().await;
        });

        ws_server.start().await
    }
}
