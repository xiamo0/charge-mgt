//! 跨 crate 共享的工具与协议。
//!
//! 目前职责：
//! - `ocpp16::CloudMessage`：cloud ↔ gateway 之间 Kafka 消息体的单一真源
//!   （任一方不应再自定义 `CloudMessage` 结构）

pub mod ocpp16;
