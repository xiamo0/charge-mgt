pub mod kafka_impl;
pub mod redis_impl;

pub use kafka_impl::{KafkaResponseChannel, MessageDirection, PendingRequest, PendingRequestTracker};
pub use redis_impl::RedisResponseChannel;

use tokio::sync::mpsc;

pub trait ResponseChannel: Send + Sync + 'static {
    fn dispatch_pending_request(
        &self,
        unique_id: String,
        charge_point_id: String,
        action: String,
        response_tx: mpsc::UnboundedSender<String>,
    );
}