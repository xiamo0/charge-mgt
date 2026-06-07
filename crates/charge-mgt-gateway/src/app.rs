//! 应用编排层，组装各子系统并启动网关服务

use crate::cloud::{ConnectionManager, KafkaConsumer, KafkaProducer};
use crate::config::{Config, ResponseChannelMode};
use crate::device::websocket::WebSocketServer;
use crate::error::Result;
use crate::response_channel::{KafkaResponseChannel, PendingRequestTracker, RedisResponseChannel, ResponseChannel};
use std::sync::Arc;
use tracing::info;

/// 网关应用实例，持有连接管理、消息生产和响应通道等核心组件
pub struct Application {
    /// 网关完整配置
    config: Config,
    /// 在线充电桩连接注册表
    connection_manager: Arc<ConnectionManager>,
    /// Kafka 消息生产者，用于上行消息发布
    kafka_producer: Arc<KafkaProducer>,
    /// 云端响应回传通道（Redis 或 Kafka 实现）
    response_channel: Arc<dyn ResponseChannel>,
    /// 待响应请求跟踪器（仅 Kafka 响应通道模式使用）
    pending_tracker: Option<Arc<PendingRequestTracker>>,
}

impl Application {
    /// 根据配置初始化 Kafka 生产者、连接管理器和响应通道
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
            "正在启动 charge-mgt-gateway 应用（响应通道={}）",
            match config.response_channel {
                ResponseChannelMode::Redis => "Redis",
                ResponseChannelMode::Kafka => "Kafka",
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

    /// 启动 WebSocket 服务和 Kafka 消费者，阻塞运行直至服务退出
    pub async fn run(&self) -> Result<()> {
        info!("正在启动 WebSocket 服务");

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