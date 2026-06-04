use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};

use crate::config::KafkaConfig;
use crate::response_channel::ResponseChannel;

use ocpp_1_6::envelope::CallError;

#[derive(Debug, Clone)]
pub enum MessageDirection {
    Upstream,
    Downstream,
}

#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub unique_id: String,
    pub charge_point_id: String,
    pub action: String,
    pub direction: MessageDirection,
    pub created_at: Instant,
    pub response_tx: mpsc::UnboundedSender<String>,
}

pub struct PendingRequestTracker {
    pending: Arc<RwLock<std::collections::HashMap<String, PendingRequest>>>,
    timeout_secs: u64,
}

impl PendingRequestTracker {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            pending: Arc::new(RwLock::new(std::collections::HashMap::new())),
            timeout_secs,
        }
    }

    pub async fn register(&self, request: PendingRequest) {
        let mut pending = self.pending.write().await;
        if pending.contains_key(&request.unique_id) {
            warn!(
                "Duplicate uniqueId {}, overwriting previous pending request",
                request.unique_id
            );
        }
        info!(
            "Registered pending request: uniqueId={}, action={}",
            request.unique_id, request.action
        );
        pending.insert(request.unique_id.clone(), request);
    }

    pub async fn remove(&self, unique_id: &str) -> Option<PendingRequest> {
        let mut pending = self.pending.write().await;
        pending.remove(unique_id)
    }

    pub async fn remove_by_charge_point(
        &self,
        charge_point_id: &str,
    ) -> Vec<PendingRequest> {
        let mut pending = self.pending.write().await;
        let keys_to_remove: Vec<String> = pending
            .values()
            .filter(|r| r.charge_point_id == charge_point_id)
            .map(|r| r.unique_id.clone())
            .collect();

        let removed: Vec<PendingRequest> = keys_to_remove
            .iter()
            .filter_map(|k| pending.remove(k))
            .collect();

        if !removed.is_empty() {
            info!(
                "Cleaned up {} pending requests for charge point {}",
                removed.len(),
                charge_point_id
            );
        }
        removed
    }

    pub fn start_timeout_eviction(self: &Arc<Self>) {
        let pending = self.pending.clone();
        let timeout_secs = self.timeout_secs;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                let mut pending_guard = pending.write().await;
                let now = Instant::now();
                let timeout = std::time::Duration::from_secs(timeout_secs);

                let timed_out_keys: Vec<String> = pending_guard
                    .values()
                    .filter(|r| now.duration_since(r.created_at) > timeout)
                    .map(|r| r.unique_id.clone())
                    .collect();

                for key in timed_out_keys {
                    if let Some(request) = pending_guard.remove(&key) {
                        warn!(
                            "Request timed out: uniqueId={}, action={}, cp={}",
                            request.unique_id, request.action, request.charge_point_id
                        );

                        match request.direction {
                            MessageDirection::Upstream => {
                                let call_error = CallError::new(
                                    &request.unique_id,
                                    "InternalError",
                                    "Cloud platform response timeout",
                                );
                                let error_json =
                                    serde_json::to_string(&call_error).unwrap_or_default();
                                request.response_tx.send(error_json).ok();
                            }
                            MessageDirection::Downstream => {
                                warn!(
                                    "Cloud command timed out waiting for CP response: action={}, cp={}",
                                    request.action, request.charge_point_id
                                );
                            }
                        }
                    }
                }
            }
        });
    }
}

pub struct KafkaResponseChannel {
    pending_tracker: Arc<PendingRequestTracker>,
}

impl KafkaResponseChannel {
    pub fn new(config: &KafkaConfig) -> Arc<Self> {
        let tracker = Arc::new(PendingRequestTracker::new(config.response_timeout_secs));
        tracker.start_timeout_eviction();
        Arc::new(Self {
            pending_tracker: tracker,
        })
    }

    pub fn pending_tracker(&self) -> Arc<PendingRequestTracker> {
        self.pending_tracker.clone()
    }
}

impl ResponseChannel for KafkaResponseChannel {
    fn dispatch_pending_request(
        &self,
        unique_id: String,
        charge_point_id: String,
        action: String,
        response_tx: mpsc::UnboundedSender<String>,
    ) {
        // Fire-and-forget: register in tracker, KafkaConsumer background task handles matching
        let tracker = self.pending_tracker.clone();
        tokio::spawn(async move {
            tracker
                .register(PendingRequest {
                    unique_id,
                    charge_point_id,
                    action,
                    direction: MessageDirection::Upstream,
                    created_at: Instant::now(),
                    response_tx,
                })
                .await;
        });
    }
}