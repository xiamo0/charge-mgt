use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, mpsc};
use tokio::time::timeout;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use super::envelope::{self, IncomingEnvelope};
use super::error::OcppError;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Clone)]
pub struct PendingInfo {
    pub action: Option<String>,
    pub at: Instant,
}

#[derive(Debug, Clone)]
pub enum IncomingEvent {
    Message(IncomingMessage),
    BadEnvelope { raw: String, error: String },
    ConnectionClosed { reason: String },
}

#[derive(Debug, Clone)]
pub enum IncomingMessage {
    CallResult {
        uid: String,
        payload: Value,
        matched_action: Option<String>,
        sent_ago: Option<Duration>,
    },
    CallError {
        uid: String,
        code: String,
        description: String,
        details: Value,
        matched_action: Option<String>,
        sent_ago: Option<Duration>,
    },
    ServerCall {
        uid: String,
        action: String,
        payload: Value,
    },
}

#[derive(Clone)]
pub struct OcppClient {
    write_tx: mpsc::UnboundedSender<String>,
    pending_user: Arc<Mutex<HashMap<String, PendingInfo>>>,
    pending_server: Arc<Mutex<HashMap<String, PendingInfo>>>,
    shutdown: CancellationToken,
    msgs_sent: Arc<std::sync::atomic::AtomicU64>,
    msgs_recv: Arc<std::sync::atomic::AtomicU64>,
}

impl OcppClient {
    pub async fn connect(
        ws_url: &str,
    ) -> Result<(Self, mpsc::UnboundedReceiver<IncomingEvent>), OcppError> {
        let (ws, _response) = connect_async(ws_url)
            .await
            .map_err(|e| OcppError::Connect(e.to_string()))?;

        let (write, read) = ws.split();
        let (write_tx, write_rx) = mpsc::unbounded_channel::<String>();
        let (event_tx, event_rx) = mpsc::unbounded_channel::<IncomingEvent>();

        let shutdown = CancellationToken::new();

        let client = Self {
            write_tx,
            pending_user: Arc::new(Mutex::new(HashMap::new())),
            pending_server: Arc::new(Mutex::new(HashMap::new())),
            shutdown: shutdown.clone(),
            msgs_sent: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            msgs_recv: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        };

        Self::spawn_writer(write, write_rx, shutdown.clone(), client.msgs_sent.clone());
        Self::spawn_reader(
            read,
            event_tx,
            shutdown.clone(),
            client.pending_user.clone(),
            client.pending_server.clone(),
            client.msgs_recv.clone(),
        );

        Ok((client, event_rx))
    }

    fn spawn_writer(
        mut write: futures_util::stream::SplitSink<WsStream, Message>,
        mut rx: mpsc::UnboundedReceiver<String>,
        shutdown: CancellationToken,
        msgs_sent: Arc<std::sync::atomic::AtomicU64>,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown.cancelled() => break,
                    msg = rx.recv() => {
                        match msg {
                            Some(text) => {
                                if write.send(Message::Text(text)).await.is_err() {
                                    break;
                                }
                                msgs_sent.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            }
                            None => break,
                        }
                    }
                }
            }
            let _ = write.close().await;
        });
    }

    fn spawn_reader(
        mut read: futures_util::stream::SplitStream<WsStream>,
        event_tx: mpsc::UnboundedSender<IncomingEvent>,
        shutdown: CancellationToken,
        pending_user: Arc<Mutex<HashMap<String, PendingInfo>>>,
        pending_server: Arc<Mutex<HashMap<String, PendingInfo>>>,
        msgs_recv: Arc<std::sync::atomic::AtomicU64>,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown.cancelled() => break,
                    frame = read.next() => {
                        match frame {
                            Some(Ok(Message::Text(text))) => {
                                msgs_recv.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                                match IncomingEnvelope::parse(&text) {
                                    Ok(IncomingEnvelope::Call { uid, action, payload }) => {
                                        pending_server.lock().await.insert(
                                            uid.clone(),
                                            PendingInfo { action: Some(action.clone()), at: Instant::now() },
                                        );
                                        let _ = event_tx.send(IncomingEvent::Message(
                                            IncomingMessage::ServerCall { uid, action, payload },
                                        ));
                                    }
                                    Ok(IncomingEnvelope::CallResult { uid, payload }) => {
                                        let matched = pending_user.lock().await.remove(&uid);
                                        let _ = event_tx.send(IncomingEvent::Message(
                                            IncomingMessage::CallResult {
                                                uid,
                                                payload,
                                                matched_action: matched.as_ref().and_then(|p| p.action.clone()),
                                                sent_ago: matched.map(|p| p.at.elapsed()),
                                            },
                                        ));
                                    }
                                    Ok(IncomingEnvelope::CallError { uid, code, description, details }) => {
                                        let matched = pending_user.lock().await.remove(&uid);
                                        let _ = event_tx.send(IncomingEvent::Message(
                                            IncomingMessage::CallError {
                                                uid,
                                                code,
                                                description,
                                                details,
                                                matched_action: matched.as_ref().and_then(|p| p.action.clone()),
                                                sent_ago: matched.map(|p| p.at.elapsed()),
                                            },
                                        ));
                                    }
                                    Err(e) => {
                                        let _ = event_tx.send(IncomingEvent::BadEnvelope {
                                            raw: text,
                                            error: e.to_string(),
                                        });
                                    }
                                }
                            }
                            Some(Ok(Message::Close(frame))) => {
                                let reason = frame
                                    .as_ref()
                                    .map(|f| f.reason.to_string())
                                    .unwrap_or_else(|| "closed".into());
                                let _ = event_tx.send(IncomingEvent::ConnectionClosed { reason });
                                break;
                            }
                            Some(Ok(_)) => {}
                            Some(Err(e)) => {
                                let _ = event_tx.send(IncomingEvent::ConnectionClosed {
                                    reason: e.to_string(),
                                });
                                break;
                            }
                            None => {
                                let _ = event_tx.send(IncomingEvent::ConnectionClosed {
                                    reason: "stream ended".into(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn send_call(
        &self,
        action: &str,
        payload: Value,
        uid: Option<String>,
    ) -> Result<String, OcppError> {
        let uid = uid.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let envelope = json!([2, &uid, action, payload]);
        let text = serde_json::to_string(&envelope).map_err(|e| OcppError::Parse(e.to_string()))?;
        self.pending_user.lock().await.insert(
            uid.clone(),
            PendingInfo {
                action: Some(action.to_string()),
                at: Instant::now(),
            },
        );
        self.write_tx
            .send(text)
            .map_err(|_| OcppError::ConnectionClosed)?;
        Ok(uid)
    }

    pub async fn send_raw(&self, text: &str) -> Result<Option<String>, OcppError> {
        match envelope::parse_raw_header(text) {
            Ok((2, uid, action)) => {
                self.pending_user.lock().await.insert(
                    uid.clone(),
                    PendingInfo {
                        action,
                        at: Instant::now(),
                    },
                );
                self.write_tx
                    .send(text.to_string())
                    .map_err(|_| OcppError::ConnectionClosed)?;
                Ok(Some(uid))
            }
            Ok(_) => {
                self.write_tx
                    .send(text.to_string())
                    .map_err(|_| OcppError::ConnectionClosed)?;
                Ok(None)
            }
            Err(e) => Err(OcppError::Parse(e.to_string())),
        }
    }

    pub async fn respond(&self, uid: &str, payload: Value) -> Result<(), OcppError> {
        {
            let mut m = self.pending_server.lock().await;
            if m.remove(uid).is_none() {
                return Err(OcppError::UnknownServerCall(uid.to_string()));
            }
        }
        let envelope = json!([3, uid, payload]);
        let text = serde_json::to_string(&envelope).map_err(|e| OcppError::Parse(e.to_string()))?;
        self.write_tx
            .send(text)
            .map_err(|_| OcppError::ConnectionClosed)
    }

    pub async fn send_error(
        &self,
        uid: &str,
        code: &str,
        description: &str,
    ) -> Result<(), OcppError> {
        {
            let mut m = self.pending_server.lock().await;
            if m.remove(uid).is_none() {
                return Err(OcppError::UnknownServerCall(uid.to_string()));
            }
        }
        let envelope = json!([4, uid, code, description, json!({})]);
        let text = serde_json::to_string(&envelope).map_err(|e| OcppError::Parse(e.to_string()))?;
        self.write_tx
            .send(text)
            .map_err(|_| OcppError::ConnectionClosed)
    }

    pub async fn pending_server_list(&self) -> Vec<(String, PendingInfo)> {
        self.pending_server
            .lock()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub async fn pending_user_count(&self) -> usize {
        self.pending_user.lock().await.len()
    }

    pub fn msgs_sent(&self) -> u64 {
        self.msgs_sent.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn msgs_recv(&self) -> u64 {
        self.msgs_recv.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }
}

pub async fn send_call_with_timeout(
    client: &OcppClient,
    action: &str,
    payload: Value,
    dur: Duration,
) -> Result<String, OcppError> {
    let fut = client.send_call(action, payload, None);
    match timeout(dur, fut).await {
        Ok(r) => r,
        Err(_) => Err(OcppError::ConnectionClosed),
    }
}
