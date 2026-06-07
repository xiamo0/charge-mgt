//! 设备接入模块
//!
//! 通过 WebSocket 接收充电桩 OCPP 消息，解析后转发至云端。

pub mod connection;
pub mod websocket;
