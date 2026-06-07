//! 云端响应回传通道
//!
//! 抽象云端异步响应的接收方式，支持两种实现：
//! - **Redis**：通过 BLPOP 阻塞等待 `resp:{uniqueId}` 键
//! - **Kafka**：通过响应主题 + PendingRequestTracker 匹配 uniqueId

pub mod kafka_impl;
pub mod redis_impl;

pub use kafka_impl::{
    KafkaResponseChannel, MessageDirection, PendingRequest, PendingRequestTracker,
};
pub use redis_impl::RedisResponseChannel;

use tokio::sync::mpsc;

/// 响应通道 trait，负责注册待响应请求并在云端回复后回传充电桩
pub trait ResponseChannel: Send + Sync + 'static {
    /// 注册一个需要等待云端响应的请求
    fn dispatch_pending_request(
        &self,
        unique_id: String,
        charge_point_id: String,
        action: String,
        response_tx: mpsc::UnboundedSender<String>,
    );
}
