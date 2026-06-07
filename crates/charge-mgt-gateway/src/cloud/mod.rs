//! 云端通信模块
//!
//! 包含 REST API 客户端、Kafka 生产/消费、消息格式定义和连接管理。

pub mod client;
pub mod consumer;
pub mod kafka;
pub mod message;

pub use client::CloudApiClient;
pub use consumer::{ConnectionManager, ConnectionMeta, KafkaConsumer};
pub use kafka::{KafkaProducer, MockKafkaProducer};
pub use message::{CloudMessage, CloudMessageInput};