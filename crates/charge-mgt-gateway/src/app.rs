use crate::cloud::MockKafkaProducer;
use crate::config::Config;
use crate::device::websocket::WebSocketServer;
use crate::error::Result;
use crate::state::AppState;
use std::sync::Arc;
use tracing::info;

pub struct Application {
    config: Config,
    state: Arc<AppState>,
    kafka_producer: Arc<MockKafkaProducer>,
}

impl Application {
    pub fn new(config: Config) -> Result<Self> {
        let kafka_producer = Arc::new(MockKafkaProducer::new(&config.kafka));

        info!("Starting charge-mgt-gateway application");

        Ok(Self {
            config,
            state: Arc::new(AppState::new()),
            kafka_producer,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting WebSocket server");

        let ws_server = WebSocketServer::new(
            self.config.device.clone(),
            self.state.clone(),
            self.kafka_producer.clone(),
            self.config.gateway.id.clone(),
            self.config.gateway.host.clone(),
        );

        ws_server.start().await
    }
}
