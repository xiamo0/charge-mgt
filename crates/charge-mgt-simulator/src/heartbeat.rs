use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use serde_json::json;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::ocpp::client::{OcppClient, send_call_with_timeout};

pub struct HeartbeatScheduler {
    cancel: CancellationToken,
    interval: Duration,
    sent_count: Arc<AtomicU64>,
    last_sent: Arc<Mutex<Option<Instant>>>,
}

impl HeartbeatScheduler {
    pub fn start(client: OcppClient, interval_secs: u64) -> Self {
        let cancel = CancellationToken::new();
        let sent_count = Arc::new(AtomicU64::new(0));
        let last_sent = Arc::new(Mutex::new(None));

        tokio::spawn({
            let client = client.clone();
            let cancel = cancel.clone();
            let sent_count = sent_count.clone();
            let last_sent = last_sent.clone();
            async move {
                let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
                // 第一个 tick 是立即，跳过它
                ticker.tick().await;
                loop {
                    tokio::select! {
                        _ = cancel.cancelled() => break,
                        _ = ticker.tick() => {
                            let payload = json!({});
                            match send_call_with_timeout(&client, "Heartbeat", payload, Duration::from_secs(5)).await {
                                Ok(_) => {
                                    sent_count.fetch_add(1, Ordering::SeqCst);
                                    *last_sent.lock().await = Some(Instant::now());
                                }
                                Err(_) => {
                                    // 不 panic，留给用户看到下一次 tick 是否恢复
                                }
                            }
                        }
                    }
                }
            }
        });

        Self {
            cancel,
            interval: Duration::from_secs(interval_secs),
            sent_count,
            last_sent,
        }
    }

    pub fn stop(&self) {
        self.cancel.cancel();
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn sent_count(&self) -> u64 {
        self.sent_count.load(Ordering::SeqCst)
    }

    pub fn last_sent(&self) -> Option<Instant> {
        // best-effort: 不持有 mutex
        self.last_sent.try_lock().ok().and_then(|g| *g)
    }
}
