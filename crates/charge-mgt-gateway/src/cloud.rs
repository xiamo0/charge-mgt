//! 云端通信模块
//!
//! 包含 REST API 客户端、Kafka 生产/消费、连接管理。
//!
//! `CloudMessage`/`CloudMessageInput` 来自 `charge-mgt-common::ocpp16`（单一真源），
//! 本 crate 不再独立定义消息类型。

pub mod client;
pub mod consumer;
pub mod kafka;

pub use charge_mgt_common::ocpp16::{CloudMessage, CloudMessageInput};
pub use client::CloudApiClient;
pub use consumer::{ConnectionManager, ConnectionMeta, KafkaConsumer};
pub use kafka::{KafkaProducer, MockKafkaProducer};
