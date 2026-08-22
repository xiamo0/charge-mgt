//! Charge-mgt Gateway
//!
//! 充电桩管理网关，负责：
//! - 通过 WebSocket 接收充电桩 OCPP 1.6 消息
//! - 将消息转发至 Kafka 供云端处理
//! - 通过 Redis 或 Kafka 接收云端响应并回传充电桩

pub mod app;
pub mod cloud;
pub mod config;
pub mod device;
pub mod error;
pub mod response_channel;
pub mod security;
pub mod state;
